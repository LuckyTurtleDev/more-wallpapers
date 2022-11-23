use crate::{load_env_var, Environment, WallpaperBuilder, WallpaperError};

mod kde;
mod x11;

#[cfg(feature = "wallpaper")]
mod wallpaper_crate;

fn get_environment() -> Result<Environment, WallpaperError> {
	let desktop = load_env_var("XDG_CURRENT_DESKTOP")?.to_lowercase();
	if desktop.as_str() == "kde" {
		return Ok(Environment::Kde);
	}
	let sessinon_type = load_env_var("XDG_SESSION_TYPE")?.to_lowercase();
	match sessinon_type.as_str() {
		"x11" => Ok(Environment::X11),
		#[cfg(feature = "wallpaper")]
		"wayland" => Ok(Environment::LinuxWallpaperCrate),
		_ => Err(WallpaperError::Unsuported(format!("{desktop} ({sessinon_type})"))),
	}
}

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let environment = get_environment()?;
	let screens = match environment {
		Environment::Kde => kde::get_screens()?,
		Environment::X11 => x11::get_screens()?,
		#[cfg(feature = "wallpaper")]
		Environment::LinuxWallpaperCrate => wallpaper_crate::get_screens(),
		#[cfg(feature = "wallpaper")]
		Environment::Windows => panic!(),
		#[cfg(feature = "wallpaper")]
		Environment::MacOS => panic!(),
	};
	Ok(WallpaperBuilder { environment, screens })
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) -> Result<(), WallpaperError> {
	match builder.environment {
		Environment::Kde => kde::set_screens(builder.screens)?,
		Environment::X11 => x11::set_screens(builder.screens)?,
		#[cfg(feature = "wallpaper")]
		Environment::LinuxWallpaperCrate => wallpaper_crate::set_screens(builder.screens)?,
		#[cfg(feature = "wallpaper")]
		Environment::Windows => panic!(),
		#[cfg(feature = "wallpaper")]
		Environment::MacOS => panic!(),
	}
	Ok(())
}
