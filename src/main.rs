use std::{time::Duration, path::Path};
use dbus::{blocking::{Connection, stdintf::org_freedesktop_dbus::Properties}, message::MatchRule, ffidisp::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged, Message};
use clap::{Parser, command};

const CONFIG_FILE: &str = "/etc/power-profiles-cfg/profiles.ron";

mod profile;
mod sysfs_interface;

#[derive(Debug)]
pub enum AppError {
  ErrorReadingFile(String)
}

/// Configurable power profiles for power-profiles-daemon
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  /// Initialize service
  #[arg(short, long)]
  init: bool,
  /// Forcefully re-apply profile configuration
  #[arg(short, long)]
  force: bool,
  /// Reload states (AC on/off, etc)
  #[arg(short, long)]
  reload: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cli = Cli::parse();
  let config_path = Path::new(CONFIG_FILE);
  let conn = Connection::new_system()?;
  let proxy = conn.with_proxy("net.hadess.PowerProfiles", "/net/hadess/PowerProfiles", Duration::from_millis(5000));
  let active_profile: String = proxy.get("net.hadess.PowerProfiles", "ActiveProfile")?;
  let profile_manager = profile::ProfileManager::new(config_path.to_path_buf());

  let profiles = profile_manager.read_profiles(&conn).expect("No profiles exist");

  // Only apply profile on startup if the `init` or `force` arg was passed
  if let Some(profile) = profiles.get(&active_profile) {
    if cli.init || cli.force {
      profile.apply_profile();
    }
  }

  // Only initialize service if the `init` arg was passed
  if !cli.init {
    return Ok(())
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