use std::{path::PathBuf, collections::HashMap, time::Duration, fs};
use dbus::{blocking::{Connection, stdintf::org_freedesktop_dbus::Properties}, arg};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

use crate::sysfs_interface;

pub type Profiles = HashMap<String, ProfileConfig>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProfileConfig {
  pub driver: String,
  pub turbo: bool,
  pub governor: String
}

impl ProfileConfig {
  fn new(driver: String, turbo: bool, governor: String) -> Self {
    Self { driver, turbo, governor }
  }

  pub fn apply_profile(&self) {
    // Turbo
    sysfs_interface::write_turbo_state(self);
    // Governor
    sysfs_interface::write_governor_state(self);
  }
}

pub struct ProfileManager {
  config_path: PathBuf
}

impl ProfileManager {
  pub fn new(config_path: PathBuf) -> Self {
    Self { config_path }
  }

  /// Returns the power profiles.
  /// 
  /// The first argument is a reference to the D-Bus [`Connection`].
  /// 
  /// # Panics
  /// 
  /// The function will panic if there was an error parsing the configuration file.
  pub fn read_profiles(&self, conn: &Connection) -> Option<Profiles> {
    let profiles = match fs::read_to_string(&self.config_path) {
      Ok(s) => {
        let profile_configs: HashMap<String, ProfileConfig> = ron::from_str(s.as_str()).expect("Error parsing RON from file");
        Some(profile_configs)
      }
      Err(_) => None
    };

    profiles.or_else(|| {
      let initial_profiles = self.read_initial_profiles(conn);
      if let Some(ref profiles) = initial_profiles {
        _ = self.save_profiles(profiles);
      }
      initial_profiles
    })
  }

  fn read_initial_profiles(&self, conn: &Connection) -> Option<Profiles> {
    let proxy = conn.with_proxy("net.hadess.PowerProfiles", "/net/hadess/PowerProfiles", Duration::from_millis(5000));
    let read_profiles: Vec<arg::PropMap> = proxy.get("net.hadess.PowerProfiles", "Profiles").ok()?;
    let mut profiles: Profiles = HashMap::new();

    for p in read_profiles {
      let profile = p.get("Profile")?;
      let driver = p.get("Driver")?;
      let profile = profile.0.as_str()?;
      let driver = driver.0.as_str()?;
      let turbo = sysfs_interface::read_turbo_state(driver).unwrap_or(true);
      let governor = sysfs_interface::read_governor_state().unwrap_or(String::from("ondemand"));

      profiles.insert(profile.to_string(), ProfileConfig::new(driver.to_string(), turbo, governor));
    }

    if profiles.is_empty() {
      return None
    }
    
    Some(profiles)
  }

  fn save_profiles(&self, profiles: &Profiles) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = self.config_path.parent().unwrap();
    let pretty_config = PrettyConfig::new().indentor("  ".to_string());
    let profiles_ron = ron::ser::to_string_pretty(profiles, pretty_config)?;

    if !config_dir.is_dir() {
      fs::create_dir(config_dir)?;
    }

    fs::write(&self.config_path, profiles_ron)?;

    Ok(())
  }
}