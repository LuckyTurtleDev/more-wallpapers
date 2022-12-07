use crate::{load_env_var, Environment, WallpaperBuilder, WallpaperError};

mod kde;
mod x11;

#[cfg(feature = "fallback")]
mod wallpaper_crate;

fn get_environment() -> Result<Environment, WallpaperError> {
	#[cfg(feature = "fallback")]
	{
		//if the SWAYSOCK env exist sawy is the active desktop
		let sway_sock = load_env_var("SWAYSOCK");
		if sway_sock.is_ok() {
			return Ok(Environment::LinuxFallback);
		}
	}
	let desktop = load_env_var("XDG_CURRENT_DESKTOP")?.to_lowercase();
	if desktop.as_str() == "kde" {
		return Ok(Environment::Kde);
	}
	let sessinon_type = load_env_var("XDG_SESSION_TYPE")?.to_lowercase();
	match sessinon_type.as_str() {
		"x11" => Ok(Environment::X11),
		#[cfg(feature = "fallback")]
		"wayland" => match desktop.as_str() {
			"budgie:gnome" => Ok(Environment::LinuxFallback), //same enviroment like gnome
			"deepin" => Ok(Environment::LinuxFallback),
			"gnome" => Ok(Environment::LinuxFallback),
			"lxde" => Ok(Environment::LinuxFallback),
			"matex" => Ok(Environment::LinuxFallback),
			"xfce" => Ok(Environment::LinuxFallback),
			_ => Err(WallpaperError::Unsuported(format!("{desktop} ({sessinon_type})"))),
		},
		_ => Err(WallpaperError::Unsuported(format!("{desktop} ({sessinon_type})"))),
	}
}

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let environment = get_environment()?;
	let screens = match environment {
		Environment::Kde => kde::get_screens()?,
		Environment::X11 => x11::get_screens()?,
		#[cfg(feature = "fallback")]
		Environment::LinuxFallback => wallpaper_crate::get_screens(),
		#[cfg(feature = "fallback")]
		Environment::Windows => panic!(),
		#[cfg(feature = "fallback")]
		Environment::MacOS => panic!(),
	};
	Ok(WallpaperBuilder { environment, screens })
}

pub(crate) fn set_screens_from_builder(builder: WallpaperBuilder) -> Result<(), WallpaperError> {
	match builder.environment {
		Environment::Kde => kde::set_screens(builder.screens)?,
		Environment::X11 => x11::set_screens(builder.screens)?,
		#[cfg(feature = "fallback")]
		Environment::LinuxFallback => wallpaper_crate::set_screens(builder.screens)?,
		#[cfg(feature = "fallback")]
		Environment::Windows => panic!(),
		#[cfg(feature = "fallback")]
		Environment::MacOS => panic!(),
	}
	Ok(())
}
