use super::check_command_error;
use crate::{
	error::{CommandError, WallpaperError},
	linux::{run, x11},
	Mode, Screen,
};
use serde::Deserialize;
use std::{io::StdoutLock, process::Command};

#[derive(Deserialize, Debug)]
struct OutputScreens {
	id: usize,
	current_mode: String,
	active: bool,
}

#[derive(Deserialize, Debug)]
struct OutputMode {
	widht: usize,
	height: usize,
}

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	let mut command = Command::new("swaymsg");
	command.args(["-t", "get_outputs"]);
	let output = check_command_error(command.output(), "swaymsg")?;
	let output = String::from_utf8(output).unwrap();
	println!("{output}");
	let output: Vec<OutputScreens> = serde_json::from_str(&output)?;
	println!("{output:#?}");
	todo!() //Ok(screens)
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), WallpaperError> {
	Ok(())
}
