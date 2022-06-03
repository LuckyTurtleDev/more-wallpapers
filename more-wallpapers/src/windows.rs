use crate::{Enviroment, Screen, WallpaperBuilder};

pub(crate) fn get_builder() -> WallpaperBuilder {
	let screens = vec![Screen {
		name: "Unknow".into(),
		wallpaper: None,
		mode: None,
	}];
	WallpaperBuilder {
		enviroment: Enviroment::WINDOWS,
		screens,
	}
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) {
	wallpaper::set_from_path(builder.screens[0].wallpaper.as_ref().unwrap());
}
