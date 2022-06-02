use clap::Parser;
use more_wallpapers::{Mode, WallpaperBuilder};

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
fn list() {
	let builder = WallpaperBuilder::new();
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
}

fn set(images: Vec<String>) {
	more_wallpapers::set_wallpapers_from_vec(images, Mode::Crop)
}

fn main() {
	match Opt::parse() {
		Opt::List => list(),
		Opt::Set(opt) => set(opt.images),
	}
}
