#![allow(clippy::tabs_in_doc_comments)]
#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]

//! Yet another wallpaper crate, which can set a wallpapers per screen.
//!
//! The main feature over other crates like [wallpaper] or [wall] is the ability to set **different wallpapers** on different screens.
//! Currently this feature is only implemented for some environments.
//! Because of this you can enable the "wallpaper" feature,
//! which uses the [wallpaper] crate as a fallback on unsupported environments.
//! This means you can use the additonal features of this crate and
//! still support the large amount of supported environments of the [wallpaper] crate.
//!
//! Currently the following environments are supported:
//!
//! | environment | set wallpaper | set wallpaper per screen | requirements |
//! --- | :---: | :---:| --- |
//! |Windows                     | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |MacOS                       | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |X11                         | ✅ | ✅ | [xwallpaper], [libxrandr]²|
//! |Budgie(wayland)             | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |Deepin(wayland)             | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |GNOME(wayland)              | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |KDE                         | ✅ | ✅ | [dbus]²|
//! |Mate(wayland)               | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |Sway                        | ✅ | ❌ | `features=["wallpaper"]`¹ |
//! |some other wayland desktops | ✅ | ❌ | `features=["wallpaper"]`¹, [swaybg], dektop must support wlr-layer-shell protocol and wl_output version 4 |
//!
//! ¹ Please check also the requirements of the [wallpaper] crate.<br/>
//! ² normally already installed.
//!
//! The information about the currently supported features are also provided by the [`Environment`] enum.
//!
//! ## QuickStart / Examples:
//! If you would like to set only a different wallpaper for each screen and don't care
//! which wallpaper is used on which screen,
//! you can use [`set_wallpapers_from_vec()`] or [`set_random_wallpapers_from_vec()`]:
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use more_wallpapers::Mode;
//!
//! let images = vec!["1.jpg", "/usr/share/wallpapers/2.jpg"];
//! more_wallpapers::set_wallpapers_from_vec(images, "default.jpg", Mode::Crop)?;
//! # Ok(())}
//! ```
//! The `"default.jpg"` is used as wallpaper for [inactive screens](Screen::active).
//! If you do not know witch value you shoud use here, you can simple use the first elment of the images vec.
//!
//! For advanced wallpaper settings you can use the [`WallpaperBuilder`]:
#![doc = doc_WallpaperBuilder_example!()]
//!  [wallpaper]: https://crates.io/crates/wallpaper
//!  [wall]: https://crates.io/crates/wall
//!  [xwallpaper]: https://github.com/stoeckmann/xwallpaper
//!  [libxrandr]: https://gitlab.freedesktop.org/xorg/app/xrandr
//!  [dbus]: https://gitlab.freedesktop.org/dbus/dbus
//!  [swaybg]: https://github.com/swaywm/swaybg

macro_rules! doc_WallpaperBuilder_example {
	() => {
		r#"``` no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use more_wallpapers::{Mode, WallpaperBuilder};

let fallback_images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
let mut i = 0;
WallpaperBuilder::new()?.set_wallpapers(|screen| {
	i += 1;
	if i == 1 {
		return ("first.jpg".to_owned(), Mode::default());
	}
	if screen.name == "HDMI1" {
		return ("/usr/share/wallpapers/hdmi.jpg".to_owned(), Mode::Fit);
	}
	(
		fallback_images[i % fallback_images.len()].to_owned(),
		Mode::Tile,
	)
})?;
# Ok(())}
```
"#
	};
}

macro_rules! doc_set_wallpapers_from_vec {
		(fn) => {
			concat!(doc_set_wallpapers_from_vec!(@private head), "set_wallpapers_from_vec", doc_set_wallpapers_from_vec!(@private body),
		doc_set_wallpapers_from_vec!(@private tail))
		};
		(builder) => {
			concat!(doc_set_wallpapers_from_vec!(@private head), "WallpaperBuilder", doc_set_wallpapers_from_vec!(@private body), "WallpaperBuilder::new()?.",
		doc_set_wallpapers_from_vec!(@private tail))
		};
		(@private head) => {
			r#"
Set the background of all screens to the wallpapers of `wallpapers`.
The wallpaper of `screen[i]` will be set to `wallpapers[i mod wallpapers.len()]`.
The `default_wallpaper` param is used if the given `wallpapers` vec is empty and as wallpaper for [inactive screens](Screen::active).
Return a vec, with dose inlcude the path of the Wallpapers,
witch was set as background.
If the same wallpaper was set multiple times to different screens,
the return value does also include the wallpaper multiple times.
```no_run
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use more_wallpapers::{"#
		};
		(@private body) => {
			r#", Mode};

let images = vec!["1.jpg", "/usr/share/wallpapers/2.jpg"];
let used_wallpapers = "#
		};
		(@private tail) => {
			r#"set_wallpapers_from_vec(images, "default.png", Mode::Crop)?;
println!("background was set to the following wallpapers {used_wallpapers:?}");
# Ok(())}
```"#
		};
	}

use camino::{Utf8Path, Utf8PathBuf};
use std::io;
use strum_macros::{Display, EnumString};

pub mod error;
#[cfg(target_os = "linux")]
use error::load_env_var;
use error::{Context, WallpaperError};

#[cfg(feature = "rand")]
use rand::{prelude::IteratorRandom, seq::SliceRandom};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use crate::linux::*;

#[cfg(all(target_os = "windows", not(feature = "wallpaper")))]
std::compile_error!("Windows does need the \"wallpaper\" feature");
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(all(target_os = "macos", not(feature = "wallpaper")))]
std::compile_error!("MacOS does need the \"wallpaper\" feature");
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use crate::macos::*;

/// define how the wallpaper will be stretch, zoom, repeated etc
#[derive(Debug, Clone, Copy, Default, EnumString, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Mode {
	///center image witout zooming. Image is may not full visible. Empty space is filled with black.
	Center,
	///zoom image to match x and y size of display and keep aspect ratio. Some parts of the image is may cut off.
	#[default]
	Crop,
	///zoom image to match match x or y size of the display, the other will be filled with a black bar at each side. All parts of the immages are visible.
	Fit,
	///zoom x and y independently from each other to match display size.
	Stretch,
	///Repeat the image until the Screen is filled. May not all parts of the image are visible.
	Tile,
}

/// Represent the used operating system or dekstop.
/// Inform about supported features, at the curren environment.
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
	Kde,
	#[cfg(feature = "wallpaper")]
	LinuxWallpaperCrate,
	#[cfg(feature = "wallpaper")]
	MacOS,
	#[cfg(feature = "wallpaper")]
	Windows,
	X11,
}
impl Environment {
	///return true, if the current environment does support various wallpaper on each screen
	pub fn support_various_wallpaper(&self) -> bool {
		match self {
			Self::Kde => true,
			#[cfg(feature = "wallpaper")]
			Self::LinuxWallpaperCrate => false,
			#[cfg(feature = "wallpaper")]
			Self::MacOS => false,
			#[cfg(feature = "wallpaper")]
			Self::Windows => false,
			Self::X11 => true,
		}
	}
}

/// include information about a connected screen
#[derive(Clone, Debug)]
pub struct Screen {
	pub name: String,
	/// current wallpaper of the screen
	pub wallpaper: Option<Utf8PathBuf>,
	/// current mode of the screen
	pub mode: Option<Mode>,
	/// indicates if screen is active.
	/// A inactive screen is current disconneted or repesents a default for new connected screens or is a fallback after restart
	pub active: bool,
}

///Builder for advance Wallpaper settings and informations.
///This struct should not be stored for a long time, because it can become outdated if the user connect or disconnect monitors or change the Display settings.
#[derive(Debug)]
pub struct WallpaperBuilder {
	screens: Vec<Screen>,
	environment: Environment,
}

impl WallpaperBuilder {
	pub fn new() -> Result<Self, WallpaperError> {
		get_builder()
	}

	///Return the count of active screens. This does not include disable screens.
	pub fn screen_count(&self) -> usize {
		self.screens.len()
	}

	///Return the count of active screens. This does not include disable screens.
	pub fn active_screen_count(&self) -> usize {
		self.screens.iter().filter(|screen| screen.active).count()
	}

	///Return the current Destkop environment.
	pub fn environment(&self) -> Environment {
		self.environment
	}

	pub fn screens(&self) -> Vec<String> {
		let mut screens = Vec::new();
		for screen in &self.screens {
			screens.push(screen.name.clone())
		}
		screens
	}

	///Set background to wallpapers, witch will be selected by the given closure.
	///The index oft screen and the current screen are passed to the closure.x
	#[doc = doc_WallpaperBuilder_example!()]
	pub fn set_wallpapers<F, P>(mut self, mut f: F) -> Result<(), WallpaperError>
	where
		P: AsRef<Utf8Path>,
		F: FnMut(&Screen) -> (P, Mode),
	{
		for mut screen in self.screens.iter_mut() {
			let tuple = f(screen);
			let path = tuple.0.as_ref();
			let path = path.canonicalize_utf8().context(path)?;
			if !path.exists() {
				return Err(io::Error::from(io::ErrorKind::NotFound)).context(path);
			}
			screen.wallpaper = Some(path);
			screen.mode = Some(tuple.1)
		}
		set_screens_from_builder(self)
	}

	#[doc = doc_set_wallpapers_from_vec!(builder)]
	pub fn set_wallpapers_from_vec<P>(
		self,
		wallpapers: Vec<P>,
		default_wallpaper: P,
		mode: Mode,
	) -> Result<Vec<Utf8PathBuf>, WallpaperError>
	where
		P: AsRef<Utf8Path>,
	{
		let mut used_wallpapers = Vec::new();
		let mut i = 0;
		self.set_wallpapers(|screen| {
			if !screen.active {
				return (default_wallpaper.as_ref(), mode);
			}
			let wallpaper = if wallpapers.is_empty() {
				default_wallpaper.as_ref()
			} else {
				wallpapers[i % wallpapers.len()].as_ref()
			};
			i += 1;
			used_wallpapers.push(wallpaper.to_owned());
			(wallpaper, mode)
		})?;
		Ok(used_wallpapers)
	}

	///Like [`Self::set_wallpapers_from_vec`],
	///but map the wallpapers randomly to the screens.
	///Selecting the same wallpaper multiple time will be avoid, if this is possible.
	#[cfg(feature = "rand")]
	pub fn set_random_wallpapers_from_vec<P>(
		self,
		wallpapers: Vec<P>,
		default_wallpaper: P,
		mode: Mode,
	) -> Result<Vec<Utf8PathBuf>, WallpaperError>
	where
		P: AsRef<Utf8Path>,
		P: Clone,
	{
		if wallpapers.is_empty() {
			// set_wallpapers_from_vec() will deal the empty inupt
			return self.set_wallpapers_from_vec(wallpapers, default_wallpaper, mode);
		}
		let mut rng = rand::thread_rng();
		let wallpapers = if wallpapers.len() < self.screen_count() {
			//extend vec to match length of screen_count
			let mut new_wallpapers = Vec::new();
			while new_wallpapers.len() < self.active_screen_count() {
				let count = (self.screen_count() - new_wallpapers.len()).min(wallpapers.len());
				let mut add = wallpapers.clone().into_iter().choose_multiple(&mut rng, count);
				new_wallpapers.append(&mut add);
			}
			new_wallpapers
		} else {
			wallpapers
		};
		let mut choose_wallpapers = wallpapers.into_iter().choose_multiple(&mut rng, self.screen_count());
		choose_wallpapers.shuffle(&mut rng);
		self.set_wallpapers_from_vec(choose_wallpapers, default_wallpaper, mode)
	}
}

#[doc = doc_set_wallpapers_from_vec!(fn)]
pub fn set_wallpapers_from_vec<P>(
	wallpapers: Vec<P>,
	default_wallpaper: P,
	mode: Mode,
) -> Result<Vec<Utf8PathBuf>, WallpaperError>
where
	P: AsRef<Utf8Path>,
{
	let builder = WallpaperBuilder::new()?;
	builder.set_wallpapers_from_vec(wallpapers, default_wallpaper, mode)
}

///Like [`set_wallpapers_from_vec`],
///but map the wallpapers randomly to the screens.
///Selecting the same wallpaper multiple time will be avoid, if this is possible.
#[cfg(feature = "rand")]
pub fn set_random_wallpapers_from_vec<P>(
	wallpapers: Vec<P>,
	default_wallpaper: P,
	mode: Mode,
) -> Result<Vec<Utf8PathBuf>, WallpaperError>
where
	P: AsRef<Utf8Path>,
	P: Clone,
{
	let builder = WallpaperBuilder::new()?;
	builder.set_random_wallpapers_from_vec(wallpapers, default_wallpaper, mode)
}
