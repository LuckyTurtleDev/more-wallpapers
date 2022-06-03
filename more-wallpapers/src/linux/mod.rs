use crate::{Enviroment, WallpaperBuilder};
use std::env;

mod kde;
mod wallpaper_crate;
mod x11;

fn get_enviroment() -> Enviroment {
	let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap().to_lowercase();
	match desktop.as_str() {
		"kde" => return Enviroment::Kde,
		_ => (),
	};
	let enviroment = env::var("XDG_SESSION_TYPE").unwrap().to_lowercase();
	match enviroment.as_str() {
		"x11" => Enviroment::X11,
		"wayland" => Enviroment::LinuxWallpaperCrate,
		_ => panic!(),
	}
}

pub(crate) fn get_builder() -> WallpaperBuilder {
	let enviroment = get_enviroment();
	let screens = match enviroment {
		Enviroment::Kde => kde::get_screens(),
		Enviroment::X11 => x11::get_screens(),
		Enviroment::LinuxWallpaperCrate => wallpaper_crate::get_screens(),
		Enviroment::Windows => panic!(),
		Enviroment::MacOS => panic!(),
	};
	WallpaperBuilder { enviroment, screens }
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) {
	match builder.enviroment {
		Enviroment::Kde => kde::set_screens(builder.screens),
		Enviroment::X11 => x11::set_screens(builder.screens),
		Enviroment::LinuxWallpaperCrate => wallpaper_crate::set_screens(builder.screens),
		Enviroment::Windows => panic!(),
		Enviroment::MacOS => panic!(),
	}
}
