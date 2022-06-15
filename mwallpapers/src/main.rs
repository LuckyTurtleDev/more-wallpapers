use anyhow::bail;
use clap::Parser;
use more_wallpapers::{Mode, WallpaperBuilder};

trait BoxedErrorHandling<V, E>
where
	E: std::fmt::Display,
{
	fn to_ah(self) -> anyhow::Result<V>;
}

impl<V, E> BoxedErrorHandling<V, E> for Result<V, E>
where
	E: std::fmt::Display,
{
	fn to_ah(self) -> anyhow::Result<V> {
		match self {
			Ok(value) => Ok(value),
			Err(error) => bail!("{error}"),
		}
	}
}

#[derive(Debug, Parser)]
pub struct SetOpt {
	#[clap(required = true)]
	images: Vec<String>,
}

#[derive(Debug, Parser)]
enum Opt {
	/// list avaible screens and othe iformation
	List,
	Set(SetOpt),
}
fn list() -> anyhow::Result<()> {
	let builder = WallpaperBuilder::new().to_ah()?;
	println!("enviroment: {}", builder.enviroment());
	println!(
		"support various wallpaper: {}",
		builder.enviroment().support_various_wallpaper()
	);
	print!("activescreens:");
	for screen in builder.screens() {
		print!(" {screen}");
	}
	println!();
	Ok(())
}

fn set(images: Vec<String>) -> anyhow::Result<()> {
	more_wallpapers::set_wallpapers_from_vec(images, Mode::Crop).to_ah()
}

fn main() {
	let result = match Opt::parse() {
		Opt::List => list(),
		Opt::Set(opt) => set(opt.images),
	};
	if let Err(err) = result {
		eprintln!("{err}");
		std::process::exit(1);
	}
}
