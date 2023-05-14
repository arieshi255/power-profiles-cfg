use clap::CommandFactory;

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
  let out_dir = std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);
  let mut path = out_dir.ancestors().nth(5).unwrap().to_owned();
  path.push("assets");
  std::fs::create_dir_all(&path).unwrap();

  let man = clap_mangen::Man::new(Cli::command());
  let mut buffer: Vec<u8> = Default::default();
  man.render(&mut buffer)?;

  std::fs::write(path.join("power-profiles-cfg.1"), buffer)?;

  Ok(())
}