use crate::{Enviroment, Screen, WallpaperBuilder};
use std::env;

mod x11;

pub(crate) fn get_builder() -> WallpaperBuilder {
	let enviroment = env::var("XDG_SESSION_TYPE").unwrap().to_lowercase();
	let enviroment = match enviroment.as_str() {
		"x11" => Enviroment::X11,
		"wayland" => Enviroment::Backup,
		_ => panic!(),
	};
	let screens = x11::get_screens();
	WallpaperBuilder {
		screen_count: screens.len(),
		screens,
		enviroment,
	}
}

pub(crate) fn set_screens(screens: Vec<Screen>) {
	x11::set_screens(screens);
}
