use clap::{Parser};

/// Configurable power profiles for power-profiles-daemon
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
  /// Initialize service
  #[clap(short, long)]
  pub init: bool,
  /// Forcefully re-apply profile configuration
  #[clap(short, long)]
  pub force: bool,
  /// Reload states (AC on/off, etc)
  #[clap(short, long)]
  pub reload: bool
}