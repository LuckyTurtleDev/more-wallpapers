use crate::{Mode, Screen};
use std::process::Command;
use xrandr;

pub(crate) fn get_screens() -> Vec<Screen> {
	let monitors = xrandr::XHandle::open().unwrap().monitors().unwrap(); //TODO: Error Handling
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
	screens
}

pub(crate) fn set_screens(screens: Vec<Screen>) {
	let mut command = Command::new("xwallpaper");
	for screen in screens {
		let mode = match screen.mode.unwrap() {
			Mode::Center => "center",
			Mode::Crop => "zoom",
			Mode::Fit => "maximize",
			Mode::Stretch => "stretch",
			Mode::Tile => "tile",
		};
		command.args(["--output", &screen.name, &format!("--{mode}"), &screen.wallpaper.unwrap()]);
	}
	command.spawn();
}
