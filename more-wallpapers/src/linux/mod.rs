use crate::{Enviroment, Screen, WallpaperBuilder};
use std::env;

mod kde;
mod x11;
use crate::wallpaper_crate;

pub fn get_enviroment() -> Enviroment {
	let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap().to_lowercase();
	match desktop.as_str() {
		"kde" => return Enviroment::KDE,
		_ => (),
	};
	let enviroment = env::var("XDG_SESSION_TYPE").unwrap().to_lowercase();
	match enviroment.as_str() {
		"x11" => Enviroment::X11,
		"wayland" => Enviroment::WALLPAPER_CRATE,
		_ => panic!(),
	}
}

pub(crate) fn get_builder() -> WallpaperBuilder {
	let enviroment = get_enviroment();
	let screens = match enviroment {
		Enviroment::KDE => kde::get_screens(),
		Enviroment::X11 => x11::get_screens(),
		Enviroment::WALLPAPER_CRATE => wallpaper_crate::get_screens(),
	};
	WallpaperBuilder { enviroment, screens }
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) {
	match builder.enviroment {
		Enviroment::KDE => kde::set_screens(builder.screens),
		Enviroment::X11 => x11::set_screens(builder.screens),
		Enviroment::WALLPAPER_CRATE => wallpaper_crate::set_screens(builder.screens),
	}
}
