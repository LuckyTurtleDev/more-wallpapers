use clap::Parser;
use more_wallpapers::{Mode, WallpaperBuilder};

#[derive(Debug, Parser)]
enum Opt {
	/// list avaible screens and othe iformation
	List,
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

fn main() {
	match Opt::parse() {
		Opt::List => list(),
	}
}
