use std::{time::Duration, fs::{self, OpenOptions}, collections::HashMap, path::{Path}, io::Write};
use dbus::{blocking::{Connection, stdintf::org_freedesktop_dbus::Properties}, message::MatchRule, ffidisp::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged, Message, arg};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "/etc/power-profiles-cfg/profiles.ron";

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ProfileConfig {
  driver: String,
  turbo: bool
}

impl ProfileConfig {
  fn new(driver: String) -> Self {
    Self { driver, turbo: true }
  }

  fn apply_profile(&self) {
    // turbo
    let turbo_sysfs = match &self.driver {
      d if d == "intel_pstate" => "/sys/devices/system/cpu/intel_pstate/no_turbo",
      _ => "/sys/devices/system/cpu/cpufreq/boost"
    };
    let turbo_state = if self.driver == "intel_pstate" { !self.turbo } else { self.turbo } as u8;
    let mut file = OpenOptions::new().write(true).open(turbo_sysfs).expect("Error opening sysfs file");
    file.write_all(turbo_state.to_string().as_bytes()).expect("Error writing to sysfs file");
  }
}

fn load_profiles(config_path: &Path) -> Option<HashMap<String, ProfileConfig>> {
  match fs::read_to_string(config_path) {
    Ok(s) => {
      let profile_configs: HashMap<String, ProfileConfig> = ron::from_str(s.as_str()).expect("Error parsing RON from file");
      Some(profile_configs)
    }
    Err(_) => None
  }
}

fn read_initial_profiles(conn: &Connection) -> Option<HashMap<String, ProfileConfig>> {
  let proxy = conn.with_proxy("net.hadess.PowerProfiles", "/net/hadess/PowerProfiles", Duration::from_millis(5000));
  let read_profiles: Vec<arg::PropMap> = proxy.get("net.hadess.PowerProfiles", "Profiles").ok()?;
  let mut profiles: HashMap<String, ProfileConfig> = HashMap::new();

  for p in read_profiles {
    let profile = p.get("Profile")?;
    let driver = p.get("Driver")?;
    let profile = profile.0.as_str()?;
    let driver = driver.0.as_str()?;

    profiles.insert(profile.to_string(), ProfileConfig::new(driver.to_string()));
  }

  if profiles.is_empty() {
    return None
  }
  
  Some(profiles)
}

fn save_profiles(config_path: &Path, profiles: &HashMap<String, ProfileConfig>) -> Result<(), Box<dyn std::error::Error>> {
  let config_dir = config_path.parent().unwrap();
  let pretty_config = PrettyConfig::new().indentor("  ".to_string());
  let profiles_ron = ron::ser::to_string_pretty(&profiles, pretty_config)?;

  if !config_dir.is_dir() {
    fs::create_dir(config_dir)?;
  }

  fs::write(config_path, profiles_ron)?;

  Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let config_path = Path::new(CONFIG_FILE);
  let conn = Connection::new_system()?;
  let proxy = conn.with_proxy("net.hadess.PowerProfiles", "/net/hadess/PowerProfiles", Duration::from_millis(5000));
  let active_profile: String = proxy.get("net.hadess.PowerProfiles", "ActiveProfile")?;

  // check if config file exists
  // if it exists, return it
  // if not, retrieve the available profiles from dbus and return those
  let profiles = load_profiles(config_path).or_else(|| {
    let initial_profiles = read_initial_profiles(&conn);
    if let Some(ref profiles) = initial_profiles {
      _ = save_profiles(config_path, profiles);
    }
    initial_profiles
  }).expect("No profiles exist");

  if let Some(profile) = profiles.get(&active_profile) {
    profile.apply_profile();
  }

  let rule = MatchRule::new()
    .with_type(dbus::MessageType::Signal)
    .with_interface("org.freedesktop.DBus.Properties")
    .with_member("PropertiesChanged")
    .with_path("/net/hadess/PowerProfiles");

  conn.add_match(rule, move |h: PropertiesPropertiesChanged, _: &Connection, _: &Message| {
    if let Some(active_profile) = h.changed_properties.get("ActiveProfile") {
      let Some(read_profile) = active_profile.0.as_str() else { return false };
      let Some(profile) = profiles.get(read_profile) else { return false };

      profile.apply_profile();
      return true
    }
    false
  }).expect("Expected /net/hadess/PowerProfiles path to exist");

  loop {
    conn.process(Duration::from_millis(1000))?;
  }
}