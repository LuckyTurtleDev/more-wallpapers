use crate::{load_env_var, Enviroment, WallpaperBuilder, WallpaperError};

mod kde;
mod wallpaper_crate;
mod x11;

fn get_enviroment() -> Result<Enviroment, WallpaperError> {
	let desktop = load_env_var("XDG_CURRENT_DESKTOP")?.to_lowercase();
	match desktop.as_str() {
		"kde" => return Ok(Enviroment::Kde),
		_ => (),
	};
	let enviroment = load_env_var("XDG_SESSION_TYPE")?.to_lowercase();
	match enviroment.as_str() {
		"x11" => Ok(Enviroment::X11),
		"wayland" => Ok(Enviroment::LinuxWallpaperCrate),
		enviroment => Err(WallpaperError::Unsuported(enviroment.to_owned())),
	}
}

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let enviroment = get_enviroment()?;
	let screens = match enviroment {
		Enviroment::Kde => kde::get_screens()?,
		Enviroment::X11 => x11::get_screens()?,
		Enviroment::LinuxWallpaperCrate => wallpaper_crate::get_screens(),
		Enviroment::Windows => panic!(),
		Enviroment::MacOS => panic!(),
	};
	Ok(WallpaperBuilder { enviroment, screens })
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) -> Result<(), WallpaperError> {
	match builder.enviroment {
		Enviroment::Kde => kde::set_screens(builder.screens)?,
		Enviroment::X11 => x11::set_screens(builder.screens)?,
		Enviroment::LinuxWallpaperCrate => wallpaper_crate::set_screens(builder.screens)?,
		Enviroment::Windows => panic!(),
		Enviroment::MacOS => panic!(),
	}
	Ok(())
}
