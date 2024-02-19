use super::run_command;
use crate::{Mode, Screen, WallpaperError};
use std::{collections::HashMap, ffi::OsStr, process::Command};

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	fn load_property(property: &str) -> Result<String, WallpaperError> {
		let mut command = Command::new("xfconf-query");
		command.args(["--channel", "xfce4-desktop", "-p"]);
		command.arg(format!("/backdrop/{property}"));
		let output = run_command(command)?;
		let output = String::from_utf8(output).unwrap();
		Ok(output)
	}

	let mut command = Command::new("xfconf-query");
	command.args(["--channel", "xfce4-desktop", "--list"]);
	let output = run_command(command)?;
	let output = String::from_utf8(output).unwrap();
	//	the outpult looks like the following:
	//
	//	/backdrop/screen0/monitor0/image-style
	//	/backdrop/screen0/monitor0/last-image
	//	/backdrop/screen0/monitor0/last-single-image
	//	/backdrop/screen0/monitorVirtual-1/workspace0/color-style
	//	/backdrop/screen0/monitorVirtual-1/workspace0/image-style
	//	/backdrop/screen0/monitorVirtual-1/workspace0/last-image
	//	/backdrop/screen0/monitorVirtual-1/workspace1/color-style
	//	/backdrop/screen0/monitorVirtual-1/workspace1/image-style
	//	/backdrop/screen0/monitorVirtual-1/workspace1/last-image
	let mut screens: HashMap<String, Screen> = Default::default();
	for line in output.lines().filter_map(|s| s.strip_prefix("/backdrop/")) {
		let mut split = line.split('/');
		let first = split.next();
		let second = split.next();
		let third = split.next();
		let fourth = split.next();
		if split.next().is_some() {
			//to long -> wrong key
			continue;
		}
		let (Some(first), Some(second), Some(third)) = (first, second, third) else {
			//to short -> wrong key
			continue;
		};
		let (screen_name, key_type, active) = if let Some(fourth) = fourth {
			// if name exist out of  part, the screen is active.
			// Otherwise it is default for new workspaces
			(format!("{}/{}/{}", first, second, third), fourth, true)
		} else {
			(format!("{}/{}", first, second), third, false)
		};
		if !(key_type == "last-image" || key_type == "image-style") {
			// wrong key
			continue;
		}
		let value = load_property(line)?;
		let screen = screens.entry(screen_name.clone()).or_insert_with(|| Screen {
			name: screen_name,
			wallpaper: None,
			mode: None,
			active,
		});
		if key_type == "last-image" {
			screen.wallpaper = Some(value.into());
		} else {
			let mode = match value.as_str() {
				"0" => None, //single color background is used instead of a image
				"1" => Some(Mode::Center),
				"2" => Some(Mode::Tile),
				"3" => Some(Mode::Stretch),
				"4" => Some(Mode::Fit),
				"5" => Some(Mode::Crop),
				_ => return Err(WallpaperError::UnknownMode(value)),
			};
			screen.mode = mode;
		}
	}
	Ok(screens.into_values().collect())
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), WallpaperError> {
	fn set_key<P: AsRef<OsStr>>(key: String, property: P) -> Result<(), WallpaperError> {
		let mut command = Command::new("xfconf-query");
		command.args(["--channel", "xfce4-desktop", "--set"]).arg(key).arg(property);
		run_command(command)?;
		Ok(())
	}

	for screen in screens {
		let key = format!("/backdrop/{}/last_image", screen.name);
		set_key(key, &screen.wallpaper.unwrap())?;
		let mode: u8 = match screen.mode.unwrap() {
			Mode::Center => 1,
			Mode::Tile => 2,
			Mode::Stretch => 3,
			Mode::Fit => 4,
			Mode::Crop => 5,
		};
		let key = format!("/backdrop/{}/image_style", screen.name);
		set_key(key, format!("{mode}"))?;
	}
	Ok(())
}
