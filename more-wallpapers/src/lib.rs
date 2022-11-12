#![allow(clippy::tabs_in_doc_comments)]
#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]

//! Yet another wallpaper crate, which can set a wallpapers per screen.
//!
//! The main feature over other crates like [wallpaper] or [wall] is the ability to set **different wallpapers** on different screens.
//! Currently this feature is only implemented for some enviroments.
//! Because of this you can enable the "wallpaper" feature,
//! which uses the [wallpaper] crate as a fallback on unsupported environments.
//! This means you can use the additonal features of this crate and
//! still support the large amount of supported enviroments of the [wallpaper] crate.
//!
//! Currently the following enviroments are supported:
//!
//! | enviroment | set wallpaper | set wallpaper per screen | requirements |
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
//! ¹ Please check also the requirements of the [wallpaper] crate.</br>
//! ² normally already installed.
//!
//! The information about the currently supported features are also provided by the [`Enviroment`] enum.
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
//! more_wallpapers::set_wallpapers_from_vec(images, Mode::Crop)?;
//! # Ok(())}
//! ```
//!
//! For advanced wallpaper settings you can use the [`WallpaperBuilder`]:
//! ``` no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use more_wallpapers::{Mode, WallpaperBuilder};
//!
//! let fallback_images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
//! WallpaperBuilder::new()?.set_wallapers(|i, screen| -> (String, Mode) {
//! 	if i == 0 {
//! 		return (
//! 			"first.jpg".to_owned(),
//! 			Mode::default(),
//! 		);
//! 	}
//! 	if screen.name == "HDMI1" {
//! 		return ("/usr/share/wallpapers/hdmi.jpg".to_owned(), Mode::Fit);
//! 	}
//! 	(
//! 		fallback_images[i % fallback_images.len()].to_owned(),
//! 		Mode::Tile,
//! 	)
//! })?;
//! # Ok(())}
//! ```
//!
//!  [wallpaper]: https://crates.io/crates/wallpaper
//!  [wall]: https://crates.io/crates/wall
//!  [xwallpaper]: https://github.com/stoeckmann/xwallpaper
//!  [libxrandr]: https://gitlab.freedesktop.org/xorg/app/xrandr
//!  [dbus]: https://gitlab.freedesktop.org/dbus/dbus
//!  [swaybg]: https://github.com/swaywm/swaybg

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
/// Inform about supported features, at the curren enviroment.
#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Enviroment {
	Kde,
	#[cfg(feature = "wallpaper")]
	LinuxWallpaperCrate,
	#[cfg(feature = "wallpaper")]
	MacOS,
	#[cfg(feature = "wallpaper")]
	Windows,
	X11,
}
impl Enviroment {
	///return true, if the current enviroment does support various wallpaper on each screen
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
	wallpaper: Option<Utf8PathBuf>,
	mode: Option<Mode>,
}

///Builder for advance Wallpaper settings and informations.
///This struct should not be stored for a long time, because it can become outdated if the user connect or disconnect monitors or change the Display settings.
#[derive(Clone, Debug)]
pub struct WallpaperBuilder {
	screens: Vec<Screen>,
	enviroment: Enviroment,
}

impl WallpaperBuilder {
	pub fn new() -> Result<Self, WallpaperError> {
		get_builder()
	}

	///Return the count of active screens. This does not include disable screens.
	pub fn screen_count(&self) -> usize {
		self.screens.len()
	}

	///Return the current Destkop enviroment.
	pub fn enviroment(&self) -> Enviroment {
		self.enviroment
	}

	pub fn screens(&self) -> Vec<String> {
		let mut screens = Vec::new();
		for screen in &self.screens {
			screens.push(screen.name.clone())
		}
		screens
	}

	///Set background to wallpapers, witch will be selected by the given closure.
	///The index oft screen and the current screen are passed to the closure.
	/// ``` no_run
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// use more_wallpapers::{Mode, WallpaperBuilder};
	///
	/// let fallback_images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
	/// WallpaperBuilder::new()?.set_wallapers(|i, screen| -> (String, Mode) {
	/// 	if i == 0 {
	/// 		return ("/usr/share/wallpapers/first.jpg".to_owned(), Mode::default());
	/// 	}
	/// 	if screen.name == "HDMI1" {
	/// 		return ("/usr/share/wallpapers/hdmi.jpg".to_owned(), Mode::Fit);
	/// 	}
	/// 	(fallback_images[i % fallback_images.len()].to_owned(), Mode::Tile)
	/// })?;
	/// # Ok(())}
	pub fn set_wallapers<F, P>(mut self, mut f: F) -> Result<(), WallpaperError>
	where
		P: AsRef<Utf8Path>,
		F: FnMut(usize, &Screen) -> (P, Mode),
	{
		for (i, mut screen) in self.screens.iter_mut().enumerate() {
			let tuple = f(i, screen);
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

	///Set the background of all screens to the wallpapers of `wallpapers`.
	///The wallpaper of `screen[i]` will be set to `wallpapers[i mod wallpapers.len()]`.
	/// Return a vec, with dose inlcude the path of the Wallpapers,
	/// witch was set as background.
	/// If the same wallpaper was set multiple times to different screens,
	/// the return value does also include the wallpaper multiple times.
	/// ```no_run
	/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
	/// use more_wallpapers::{Mode, WallpaperBuilder};
	///
	/// let images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
	/// let used_wallpapers = WallpaperBuilder::new()?.set_wallpapers_from_vec(images, Mode::Crop)?;
	/// println!("background was set to the following wallpapers {used_wallpapers:?}");
	/// # Ok(())}
	/// ```
	pub fn set_wallpapers_from_vec<P>(self, wallpapers: Vec<P>, mode: Mode) -> Result<Vec<Utf8PathBuf>, WallpaperError>
	where
		P: AsRef<Utf8Path>,
	{
		let mut used_wallpapers = Vec::new();
		self.set_wallapers(|i, _| {
			let wallpaper = wallpapers[i % wallpapers.len()].as_ref();
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
		mode: Mode,
	) -> Result<Vec<Utf8PathBuf>, WallpaperError>
	where
		P: AsRef<Utf8Path>,
		P: Clone,
	{
		let mut rng = rand::thread_rng();
		let wallpapers = if wallpapers.len() < self.screen_count() {
			//extend vec to match length of screen_count
			let mut new_wallpapers = Vec::new();
			while new_wallpapers.len() < self.screen_count() {
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
		self.set_wallpapers_from_vec(choose_wallpapers, mode)
	}
}

///Set the background of all screens to the wallpapers of `wallpapers`.
///The wallpaper of `screen[i]` will be set to `wallpapers[i mod wallpapers.len()]`.
/// Return a vec, with dose inlcude the path of the Wallpapers,
/// witch was set as background.
/// If the same wallpaper was set multiple times to different screens,
/// the return value does also include the wallpaper multiple times.
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use more_wallpapers::{set_wallpapers_from_vec, Mode};
///
/// let images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
/// let used_wallpapers = set_wallpapers_from_vec(images, Mode::Crop)?;
/// println!("background was set to the following wallpapers {used_wallpapers:?}");
/// # Ok(())}
/// ```
pub fn set_wallpapers_from_vec<P>(wallpapers: Vec<P>, mode: Mode) -> Result<Vec<Utf8PathBuf>, WallpaperError>
where
	P: AsRef<Utf8Path>,
{
	let builder = WallpaperBuilder::new()?;
	builder.set_wallpapers_from_vec(wallpapers, mode)
}

///Like [`set_wallpapers_from_vec`],
///but map the wallpapers randomly to the screens.
///Selecting the same wallpaper multiple time will be avoid, if this is possible.
#[cfg(feature = "rand")]
pub fn set_random_wallpapers_from_vec<P>(wallpapers: Vec<P>, mode: Mode) -> Result<Vec<Utf8PathBuf>, WallpaperError>
where
	P: AsRef<Utf8Path>,
	P: Clone,
{
	let builder = WallpaperBuilder::new()?;
	builder.set_random_wallpapers_from_vec(wallpapers, mode)
}
