use std::env;
use thiserror::Error;

#[cfg(target_os = "linux")]
use dbus;
#[cfg(target_os = "linux")]
use serde_json;
#[cfg(target_os = "linux")]
use xrandr;

#[cfg(target_os = "linux")]
#[derive(Debug, Error)]
pub enum CommandError {
	#[cfg(target_os = "linux")]
	#[error("failed to execute command: {0}")]
	CommandIO(#[from] std::io::Error),

	#[cfg(target_os = "linux")]
	#[error("{command} exit with code {exit_code}: {stderr:?}")]
	CommandStatus {
		command: &'static str,
		exit_code: i32,
		stderr: Vec<u8>,
	},
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WallpaperError {
	#[error("unsupported Destkop Enviroment {0:?}")]
	Unsuported(String),

	#[error("can not read Enviroment Variable {0:?}: {1}")]
	EnvVar(&'static str, env::VarError),

	#[cfg(target_os = "linux")]
	#[error("Dbus error: {0}")]
	Dbus(#[from] dbus::Error),

	#[cfg(target_os = "linux")]
	#[error("Dbus error: {0}")]
	SerdeJson(#[from] serde_json::Error),

	#[cfg(target_os = "linux")]
	#[error("xrandr  erro): {0}")]
	Xrandr(#[from] xrandr::XrandrError),

	#[cfg(target_os = "linux")]
	#[error("{0}")]
	Command(#[from] CommandError),

	#[error("{0}")]
	WallpaperCrate(#[from] Box<dyn std::error::Error>),
}

pub(crate) fn load_env_var(key: &'static str) -> Result<String, WallpaperError> {
	std::env::var(key).map_err(|err| WallpaperError::EnvVar(key, err))
}
