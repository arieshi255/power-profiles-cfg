use std::{fs::OpenOptions, io::{Read, Write}};

use crate::{profile::ProfileConfig, AppError};

pub fn read_governor_state() -> Result<String, AppError> {
  let mut buffer = Vec::new();
  let mut file = OpenOptions::new().read(true).open("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor").expect("Error opening sysfs file");
  _ = file.read_to_end(&mut buffer);

  String::from_utf8(buffer).map(|v| v.trim().to_string()).map_err(|e| AppError::ErrorReadingFile(e.to_string()))
}

pub fn write_governor_state(profile: &ProfileConfig) {
  let num = num_cpus::get();

  for i in 0..num {
    let mut file = OpenOptions::new().write(true)
      .open(format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_governor", i))
      .expect("Error opening sysfs file");
    file.write_all(profile.governor.as_bytes()).expect("Error writing to sysfs file");
  }
}

pub fn read_turbo_state(driver: &str) -> Result<bool, AppError> {
  let turbo_sysfs = match driver {
    d if d == "intel_pstate" => "/sys/devices/system/cpu/intel_pstate/no_turbo",
    _ => "/sys/devices/system/cpu/cpufreq/boost"
  };
  let mut buffer = Vec::new();
  let mut file = OpenOptions::new().read(true).open(turbo_sysfs).expect("Error opening sysfs file");
  _ = file.read_to_end(&mut buffer);

  let output = String::from_utf8(buffer)
    .map(|v| v.trim().to_string()).map_err(|e| AppError::ErrorReadingFile(e.to_string()))?;

  // Invert state for intel_pstate
  let state = match output.as_str() {
    "0" => driver == "intel_pstate",
    "1" => driver != "intel_pstate",
    _ => true
  };

  Ok(state)
}

pub fn write_turbo_state(profile: &ProfileConfig) {
  let turbo_sysfs = match &profile.driver {
    d if d == "intel_pstate" => "/sys/devices/system/cpu/intel_pstate/no_turbo",
    _ => "/sys/devices/system/cpu/cpufreq/boost"
  };
  let turbo_state = if profile.driver == "intel_pstate" { !profile.turbo } else { profile.turbo } as u8;
  let mut file = OpenOptions::new().write(true).open(turbo_sysfs).expect("Error opening sysfs file");
  file.write_all(turbo_state.to_string().as_bytes()).expect("Error writing to sysfs file");
}