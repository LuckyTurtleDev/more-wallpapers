use crate::{error::CommandError, load_env_var, Environment, WallpaperBuilder, WallpaperError};
use std::{ffi::OsStr, process::Command};

mod cinnamon;
mod kde;
mod sway;
mod x11;
mod xfce;

#[cfg(feature = "fallback")]
mod wallpaper_crate;

fn get_environment() -> Result<Environment, WallpaperError> {
	#[cfg(feature = "fallback")]
	{
		//if the SWAYSOCK env exist sawy is the active desktop
		let sway_sock = load_env_var("SWAYSOCK");
		if sway_sock.is_ok() {
			return Ok(Environment::Sway);
		}
	}
	let desktop = load_env_var("XDG_CURRENT_DESKTOP")?.to_lowercase();
	if desktop.as_str() == "x-cinnamon" {
		return Ok(Environment::Cinnamon);
	}
	if desktop.as_str() == "kde" {
		return Ok(Environment::Kde);
	}
	if desktop.as_str() == "xfce" {
		return Ok(Environment::Xfce);
	}
	let sessinon_type = load_env_var("XDG_SESSION_TYPE")?.to_lowercase();
	match sessinon_type.as_str() {
		"x11" => Ok(Environment::X11),
		#[cfg(feature = "fallback")]
		"wayland" => match desktop.as_str() {
			"budgie:gnome" | "deepin" | "gnome" | "lxde" | "mate" => Ok(Environment::LinuxFallback),
			_ => Err(WallpaperError::Unsuported(format!("{desktop} ({sessinon_type})"))),
		},
		_ => Err(WallpaperError::Unsuported(format!("{desktop} ({sessinon_type})"))),
	}
}

pub(crate) fn get_builder() -> Result<WallpaperBuilder, WallpaperError> {
	let environment = get_environment()?;
	let screens = match environment {
		Environment::Cinnamon => cinnamon::get_screens()?,
		Environment::Kde => kde::get_screens()?,
		Environment::Sway => sway::get_screens()?,
		Environment::X11 => x11::get_screens()?,
		Environment::Xfce => xfce::get_screens()?,
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
		Environment::Cinnamon => cinnamon::set_screens(builder.screens)?,
		Environment::Kde => kde::set_screens(builder.screens)?,
		Environment::Sway => sway::set_screens(builder.screens)?,
		Environment::X11 => x11::set_screens(builder.screens)?,
		Environment::Xfce => xfce::set_screens(builder.screens)?,
		#[cfg(feature = "fallback")]
		Environment::LinuxFallback => wallpaper_crate::set_screens(builder.screens)?,
		#[cfg(feature = "fallback")]
		Environment::Windows => panic!(),
		#[cfg(feature = "fallback")]
		Environment::MacOS => panic!(),
	}
	Ok(())
}

/// run a command, check error code and convert the result
fn run<I, S>(program: &str, args: I) -> Result<Vec<u8>, CommandError>
where
	I: IntoIterator<Item = S>,
	S: AsRef<OsStr>,
{
	let mut command = Command::new(program);
	command.args(args);
	run_command(command)
}

/// allow also checking more complex commands
fn run_command(
	mut command: Command,
) -> Result<Vec<u8>, CommandError> {
	let output = command.output();
	let output = output.map_err(|err| CommandError::CommandIO(command.get_program().into(), err))?;
	if !output.status.success() {
		return Err(CommandError::CommandStatus {
			command,
			exit_code: output.status.code(),
			stderr: output.stderr,
		});
	}
	Ok(output.stdout)
}
