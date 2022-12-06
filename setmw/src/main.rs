
use clap::Parser;
use more_wallpapers::{Mode, WallpaperBuilder};

#[derive(Debug, Parser)]
pub struct SetVecOpt {
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
	/// set all screens to the given list wallpaper(s)
	SetVec(SetVecOpt),
	/*
	/// set differnt wallpaper per screen
	Set(SetOpt),
	*/
}
fn list() -> anyhow::Result<()> {
	let builder = WallpaperBuilder::new()?;
	println!("environment: {}", builder.environment());
	println!(
		"support various wallpaper: {}",
		builder.environment().support_various_wallpaper()
	);
	print!("activescreens:");
	for screen in builder.screens() {
		print!(" {screen}");
	}
	println!();
	Ok(())
}

/*
fn set(opt: SetOpt) -> anyhow::Result<()> {
	//validate input
	if !(opt.screens.len() == opt.images.len() && (opt.screens.len() == opt.modes.len() || opt.modes.len() == 0)) {
		bail!("--screen, --image and --modes must be the same length")
	}
	let builder = WallpaperBuilder::new().to_ah()?;
	builder
		.set_wallpapers(|_, screen| -> (String, Mode) {
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
*/

fn set_vec(opt: SetVecOpt) -> anyhow::Result<()> {
	let default = opt.images.first().unwrap().to_owned();
	let used_wallpapers = more_wallpapers::set_wallpapers_from_vec(opt.images, default, Mode::Crop)?;
	println!("The backgrounds have been set to the following wallpapers {used_wallpapers:?}");
	Ok(())
}

fn main() {
	let result = match Opt::parse() {
		Opt::List => list(),
		//Opt::Set(opt) => set(opt),
		Opt::SetVec(opt) => set_vec(opt),
	};
	if let Err(err) = result {
		eprintln!("ERROR: {err}");
		std::process::exit(1);
	}
}
