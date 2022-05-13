use crate::{Enviroment, Screen, WallpaperBuilder};
use std::env;

mod x11;
use crate::wallpaper_crate;

pub(crate) fn get_builder() -> WallpaperBuilder {
	let enviroment = env::var("XDG_SESSION_TYPE").unwrap().to_lowercase();
	let tupple = match enviroment.as_str() {
		"x11" => (Enviroment::X11, x11::get_screens()),
		"wayland" => (Enviroment::WALLPAPER_CRATE, wallpaper_crate::get_screens()),
		_ => panic!(),
	};
	WallpaperBuilder {
		enviroment: tupple.0,
		screens: tupple.1,
	}
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) {
	match builder.enviroment {
		Enviroment::X11 => x11::set_screens(builder.screens),
		Enviroment::WALLPAPER_CRATE => wallpaper_crate::set_screens(builder.screens),
	}
}
