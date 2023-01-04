use crate::{
	error::{CommandError, WallpaperError},
	linux::{run, x11},
	Mode, Screen,
};
use camino::Utf8PathBuf;
use std::{
	str::FromStr,
	time::{Duration, Instant},
};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
enum CMode {
	Zoom,
	Centered,
	Wallpaper,
	Scaled,
	Stretched,
	Spanned,
}

impl From<Mode> for CMode {
	fn from(value: Mode) -> Self {
		match value {
			Mode::Crop => Self::Zoom,
			Mode::Center => Self::Centered,
			Mode::Tile => Self::Wallpaper,
			Mode::Fit => Self::Scaled,
			Mode::Stretch => Self::Stretched,
		}
	}
}

impl From<CMode> for Option<Mode> {
	fn from(value: CMode) -> Self {
		match value {
			CMode::Zoom => Some(Mode::Crop),
			CMode::Centered => Some(Mode::Center),
			CMode::Wallpaper => Some(Mode::Tile),
			CMode::Scaled => Some(Mode::Fit),
			CMode::Stretched => Some(Mode::Stretch),
			CMode::Spanned => None,
		}
	}
}

/// This is a wrapper around x11, because cinnamon does support to set different wallpaper per screen; see https://github.com/linuxmint/cinnamon/issues/2301
/// The wrapper add the abbility to set the default screen.

fn get_delault_screen() -> Result<Screen, CommandError> {
	let output = run("dconf", ["read", "/org/cinnamon/desktop/background/picture-uri"])?;
	let default_wallpaper: Option<Utf8PathBuf> = String::from_utf8(output)
		.ok()
		.map(|wallpaper| wallpaper.trim_start_matches("'file://").trim_end_matches("'\n").into());
	let output = run("dconf", ["read", "/org/cinnamon/desktop/background/picture-options"])?;
	let default_mode = String::from_utf8(output)
		.ok()
		.map(|value| {
			CMode::from_str(&value.trim_start_matches("'").trim_end_matches("'\n"))
				.ok()
				.map(|value| Option::<Mode>::from(value))
				.flatten()
		})
		.flatten();
	let screen = Screen {
		name: "default".to_owned(),
		wallpaper: default_wallpaper,
		mode: default_mode,
		active: false,
	};
	Ok(screen)
}

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	let mut screens = vec![get_delault_screen()?];
	screens.append(&mut x11::get_screens()?);
	Ok(screens)
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), WallpaperError> {
	let (x11_screens, default_screen): (Vec<Screen>, Vec<Screen>) = screens.into_iter().partition(|screen| screen.active);
	x11::set_screens(x11_screens.clone())?;
	let mut changed = false;
	if let Some(screen) = default_screen.first() {
		let current_state = get_delault_screen()?;
		if screen.mode != current_state.mode {
			changed = true;
			run("dconf", [
				"write",
				"/org/cinnamon/desktop/background/picture-options",
				&format!("'{}'", CMode::from(screen.mode.unwrap())),
			])?;
		}
		if screen.wallpaper != current_state.wallpaper {
			changed = true;
			run("dconf", [
				"write",
				"/org/cinnamon/desktop/background/picture-uri",
				&format!("'file://{}'", screen.wallpaper.as_ref().unwrap()),
			])?;
		}
	}
	//need to set wallpaper multiple time, otherwise cinnamon does overide them
	//I do not like this solution, but it does work
	//at least the time constant and is independent from the cinnamon effect speed
	if changed {
		let time = Instant::now();
		while time.elapsed() < Duration::from_millis(900) {
			x11::set_screens(x11_screens.clone())?;
		}
	}
	Ok(())
}
