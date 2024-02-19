use super::run_command;
use crate::{error::WallpaperError, Mode, Screen};
use serde::Deserialize;
use std::process::Command;
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
enum SMode {
	Stretch,
	Fill,
	Fit,
	Center,
	Tile,
}

impl From<Mode> for SMode {
	fn from(value: Mode) -> Self {
		match value {
			Mode::Center => Self::Center,
			Mode::Crop => Self::Fill,
			Mode::Fit => Self::Fit,
			Mode::Stretch => Self::Stretch,
			Mode::Tile => Self::Tile,
		}
	}
}

impl From<SMode> for Mode {
	fn from(value: SMode) -> Self {
		match value {
			SMode::Center => Self::Center,
			SMode::Fill => Self::Crop,
			SMode::Fit => Self::Fit,
			SMode::Stretch => Self::Stretch,
			SMode::Tile => Self::Tile,
		}
	}
}

#[derive(Deserialize, Debug)]
struct OutputScreens {
	name: String,
	active: bool,
}

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	let mut command = Command::new("swaymsg");
	command.args(["-t", "get_outputs"]);
	let output = run_command(command)?;
	let output = String::from_utf8(output).unwrap();
	println!("{output}");
	let output: Vec<OutputScreens> = serde_json::from_str(&output)?;
	println!("{output:#?}");
	Ok(output
		.into_iter()
		.map(|screen| Screen {
			name: screen.name,
			wallpaper: None,
			mode: None,
			active: screen.active,
		})
		.collect())
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), WallpaperError> {
	for screen in screens {
		let mut command = Command::new("swaymsg");
		command
			.arg("output")
			.arg(screen.name)
			.arg("bg")
			.arg(screen.wallpaper.unwrap())
			.arg(format!("{}", SMode::from(screen.mode.unwrap())));
		run_command(command)?;
	}
	Ok(())
}
