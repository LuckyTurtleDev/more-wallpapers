use crate::{error::CommandError, Mode, Screen};
use std::process::Command;

pub(crate) fn get_screens() -> Result<Vec<Screen>, xrandr::XrandrError> {
	let monitors = xrandr::XHandle::open()?.monitors()?;
	let mut screens = Vec::new();
	for monitor in monitors {
		if monitor.is_automatic {
			screens.push(Screen {
				name: monitor.name,
				wallpaper: None,
				mode: None,
			})
		}
	}
	Ok(screens)
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), CommandError> {
	let mut command = Command::new("xwallpaper");
	for screen in screens {
		let mode = match screen.mode.unwrap() {
			Mode::Center => "center",
			Mode::Crop => "zoom",
			Mode::Fit => "maximize",
			Mode::Stretch => "stretch",
			Mode::Tile => "tile",
		};
		command.args([
			"--output",
			&screen.name,
			&format!("--{mode}"),
			screen.wallpaper.unwrap().to_str().unwrap(),
		]);
	}
	let out = command.output()?;
	if !out.status.success() {
		let error = CommandError::CommandStatus {
			command: "xrandr",
			exit_code: out.status.code().unwrap(),
			stderr: out.stderr,
		};
		return Err(error);
	};
	Ok(())
}
