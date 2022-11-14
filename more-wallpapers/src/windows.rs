use crate::{Enviroment, Screen, WallpaperBuilder, WallpaperError};

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let screens = vec![Screen {
		name: "Unknow".into(),
		wallpaper: None,
		mode: None,
		active: true,
	}];
	Ok(WallpaperBuilder {
		enviroment: Enviroment::Windows,
		screens,
	})
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) -> Result<(), WallpaperError> {
	wallpaper::set_from_path(builder.screens[0].wallpaper.as_ref().unwrap().as_ref())?;
	Ok(())
}
