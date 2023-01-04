//dconf write "/org/cinnamon/desktop/background/picture-uri" '"file:///home/lukas/foo.jpg"'; xwallpaper --output DVI-0 --center /home/lukas/baar.png
use crate::{
	error::WallpaperError,
	linux::{run, x11},
	Mode, Screen,
};
use camino::Utf8PathBuf;
use std::str::FromStr;
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

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	let output = run("dconf", ["read", "/org/cinnamon/desktop/background/picture-uri"])?;
	let default_wallpaper: Option<Utf8PathBuf> = String::from_utf8(output)
		.ok()
		.map(|wallpaper| wallpaper.trim_start_matches("file://").into());
	let output = run("dconf", ["read", "/org/cinnamon/desktop/background/picture-options"])?;
	let default_mode = String::from_utf8(output)
		.ok()
		.map(|value| {
			CMode::from_str(&value)
				.ok()
				.map(|value| Option::<Mode>::from(value))
				.flatten()
		})
		.flatten();
	let mut screens = vec![Screen {
		name: "default".to_owned(),
		wallpaper: default_wallpaper,
		mode: default_mode,
		active: false,
	}];
	screens.append(&mut x11::get_screens()?);
	Ok(screens)
}
