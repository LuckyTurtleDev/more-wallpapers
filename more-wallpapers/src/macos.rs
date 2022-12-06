use crate::{Environment, Screen, WallpaperBuilder, WallpaperError};

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let screens = vec![Screen {
		name: "Unknow".into(),
		wallpaper: None,
		mode: None,
		active: true,
	}];
	Ok(WallpaperBuilder {
		environment: Environment::MacOS,
		screens,
	})
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) -> Result<(), WallpaperError> {
	fallback::set_from_path(builder.screens[0].wallpaper.as_ref().unwrap().as_str())?;
	Ok(())
}
