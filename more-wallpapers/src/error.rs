#[cfg(target_os = "linux")]
use std::env;
use std::io;
use thiserror::Error;

#[cfg(target_os = "linux")]
use serde_json;
#[cfg(target_os = "linux")]
use xrandr;

#[cfg(target_os = "linux")]
#[derive(Debug, Error)]
pub enum CommandError {
	#[cfg(target_os = "linux")]
	#[error("failed to execute command {0}: {1}")]
	CommandIO(&'static str, std::io::Error),

	#[cfg(target_os = "linux")]
	#[error("{command} exit with code {exit_code:?}: {stderr:?}")]
	CommandStatus {
		command: &'static str,
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
