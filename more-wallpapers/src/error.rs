#[cfg(target_os = "linux")]
use std::env;
use std::{io, process::Command, ffi::OsString};
use thiserror::Error;

#[cfg(target_os = "linux")]
use serde_json;
#[cfg(target_os = "linux")]
use xrandr;

#[cfg(target_os = "linux")]
#[derive(Debug, Error)]
pub enum CommandError {
	#[cfg(target_os = "linux")]
	#[error("failed to execute program {0:?}: {1}")]
	CommandIO(OsString, std::io::Error),

	#[cfg(target_os = "linux")]
	#[error("{command:?} exit with code {exit_code:?}:\n{}", String::from_utf8_lossy(.stderr))]
	CommandStatus {
		command: Command,
		exit_code: Option<i32>,
		stderr: Vec<u8>,
	},
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WallpaperError {
	#[cfg(target_os = "linux")]
	#[error("unsupported Desktop Environment {0:?}")]
	Unsuported(String),

	#[cfg(target_os = "linux")]
	#[error("can not read Environment Variable {0:?}: {1}")]
	EnvVar(&'static str, env::VarError),

	#[cfg(target_os = "linux")]
	#[error("Dbus error: {0}")]
	Dbus(#[from] rustbus::connection::Error),

	#[cfg(target_os = "linux")]
	#[error("failed to serialize json output: {0}")]
	SerdeJson(#[from] serde_json::Error),

	#[cfg(target_os = "linux")]
	#[error("xrandr  erro): {0}")]
	Xrandr(#[from] xrandr::XrandrError),

	#[cfg(target_os = "linux")]
	#[error("{0}")]
	Command(#[from] CommandError),

	#[cfg(feature = "fallback")]
	#[error("{0}")]
	WallpaperCrate(#[from] fallback::Error),

	#[error("{0:?} {1}")]
	IOError(String, io::Error),

	#[cfg(target_os = "linux")]
	#[error("Unknow XFCE wallpaper mode {0:?}")]
	UnknownMode(String),
}

pub(crate) trait Context<V> {
	fn context<C>(self, context: C) -> Result<V, WallpaperError>
	where
		C: std::fmt::Display;
}

impl<V> Context<V> for Result<V, io::Error> {
	fn context<C>(self, context: C) -> Result<V, WallpaperError>
	where
		C: std::fmt::Display,
	{
		self.map_err(|error| WallpaperError::IOError(context.to_string(), error))
	}
}

#[cfg(target_os = "linux")]
pub(crate) fn load_env_var(key: &'static str) -> Result<String, WallpaperError> {
	std::env::var(key).map_err(|err| WallpaperError::EnvVar(key, err))
}
