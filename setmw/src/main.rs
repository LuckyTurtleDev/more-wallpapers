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
pub struct SetAllOpt {
	#[clap(required = true)]
	images: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct SetOpt {
	#[clap(short, long, required(true))]
	screens: Vec<String>,
	#[clap(short, long, required(true))]
	images: Vec<String>,
	#[clap(short, long)]
	modes: Vec<Mode>,
}

#[derive(Debug, Parser)]
enum Opt {
	/// list avaible screens and other information
	List,
	/// set all screens to the given wallpaper(s)
	SetAll(SetAllOpt),
	/// set differnt wallpaper per screen
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

fn set(opt: SetOpt) -> anyhow::Result<()> {
	//validate input
	if !(opt.screens.len() == opt.images.len() && (opt.screens.len() == opt.modes.len() || opt.modes.len() == 0)) {
		bail!("--screen, --image and --modes must be the same length")
	}
	let builder = WallpaperBuilder::new().to_ah()?;
	builder
		.set_wallapers(|_, screen| -> (String, Mode) {
			let index = opt
				.screens
				.iter()
				.position(|r| r == &screen.name)
				.expect("no wallpaper specified for screen {screen.name}");
			(opt.screens[index].clone(), *opt.modes.get(index).unwrap_or(&Mode::default()))
		})
		.to_ah()?;
	Ok(())
}

fn set_all(opt: SetAllOpt) -> anyhow::Result<()> {
	let used_wallpapers = more_wallpapers::set_wallpapers_from_vec(opt.images, Mode::Crop).to_ah()?;
	println!("The backgrounds have been set to the following wallpapers {used_wallpapers:?}");
	Ok(())
}

fn main() {
	let result = match Opt::parse() {
		Opt::List => list(),
		Opt::Set(opt) => set(opt),
		Opt::SetAll(opt) => set_all(opt),
	};
	if let Err(err) = result {
		eprintln!("ERROR: {err}");
		std::process::exit(1);
	}
}
