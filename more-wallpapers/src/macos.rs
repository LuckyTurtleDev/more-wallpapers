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
	let screen = builder.screens.first().unwrap();
	fallback::set_from_path(screen.wallpaper.as_ref().unwrap().as_str())?;
	fallback::set_mode(screen.mode.unwrap().into())?;
	Ok(())
}
