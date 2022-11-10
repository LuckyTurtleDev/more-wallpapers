use crate::{Enviroment, Screen, WallpaperBuilder, WallpaperError};

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let screens = vec![Screen {
		name: "Unknow".into(),
		wallpaper: None,
		mode: None,
	}];
	Ok(WallpaperBuilder {
		enviroment: Enviroment::MacOS,
		screens,
	})
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) -> Result<(), WallpaperError> {
	wallpaper::set_from_path(builder.screens[0].wallpaper.as_ref().unwrap().as_str())?;
	Ok(())
}
