#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]

//! Yet another wallpaper Crate, witch can set a wallpaper per screen.
//!
//! The main feature over other crates like [wallpaper](https://crates.io/crates/wallpaper) or [wall](https://crates.io/crates/wall) is the abbilty to set **different wallpapers** on differens screens.
//! Current this feature is only implemated for some Enviroments.
//! Because of this you can enable the "wallpaper" feature,
//! with does use the [wallpaper](https://crates.io/crates/wallpaper) crate as fallback.
//! So you can use the additonal features of this crate and
//! still support the larger amount of supported Enviroments of the [wallpaper](https://crates.io/crates/wallpaper) crate.
//!
//! Current the following enviroments are supported:
//!
//! | enviroment | set wallpaper | set wallpaper per screen | requirements |
//! --- | :---: | :---:| --- |
//! |Windows                     | ✅ | ❌ | features=["wallpaper"] |
//! |MacOS                       | ✅ | ❌ | features=["wallpaper"] |
//! |X11                         | ✅ | ✅ | [xwallpaper](https://github.com/stoeckmann/xwallpaper) |
//! |Budgie(wayland)             | ✅ | ❌ | features=["wallpaper"] |
//! |Deepin(wayland)             | ✅ | ❌ | features=["wallpaper"] |
//! |GNOME(wayland)              | ✅ | ❌ | features=["wallpaper"] |
//! |KDE                         | ✅ | ✅ | [xrandr](https://gitlab.freedesktop.org/xorg/app/xrandr), [dbus](https://gitlab.freedesktop.org/dbus/dbus) |
//! |Mate(wayland)               | ✅ | ❌ | features=["wallpaper"] |
//! |Sway                        | ✅ | ❌ | features=["wallpaper"], [swaybg](https://github.com/swaywm/swaybg) |
//! |some other wayland desktops | ✅ | ❌ | features=["wallpaper"], [swaybg](https://github.com/swaywm/swaybg), dektop must support wlr-layer-shell protocol and wl_output version 4 |
//!
//! The information abot the current supported features are also provided by [`Enviroment`].
//!
//! <br/><br/>
//! **QuickStart / Examples:**<br/>
//! If you would like to set only a different wallpaper at each screen and do not care,
//! witch wallpaper should be set to witch screen,
//! you can use [`set_wallpapers_from_vec()`] or [`set_random_wallpapers_from_vec()`]:
//! ```
//! # fn main(){
//! # let _ = catch_error(); //will fail on ci, because of missing enviroment varibable and gui
//! # }
//! # fn catch_error() -> Result<(), Box<dyn std::error::Error>> {
//! use more_wallpapers::Mode;
//!
//! let images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
//! more_wallpapers::set_wallpapers_from_vec(images, Mode::Crop)?;
//! # Ok(())}
//! ```
//!
//! For advance wallpaper settings you can use the [`WallpaperBuilder`].
//! ```
//! # fn main(){
//! # let _ = catch_error();
//! # }
//! # fn catch_error() -> Result<(), Box<dyn std::error::Error>> {
//! use more_wallpapers::{Mode, WallpaperBuilder};
//!
//! let fallback_images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
//! WallpaperBuilder::new()?.set_wallapers(|i, screen| -> (String, Mode) {
//! 	if i == 0 {
//! 		return ("/usr/share/wallpapers/first.jpg".to_owned(), Mode::default());
//! 	}
//! 	if screen.name == "HDMI1" {
//! 		return ("/usr/share/wallpapers/hdmi.jpg".to_owned(), Mode::Fit);
//! 	}
//! 	(fallback_images[i % fallback_images.len()].to_owned(), Mode::Tile)
//! })?;
//! # Ok(())}
//! ```
use educe::Educe;
use strum_macros::{Display, EnumString};

pub mod error;
#[cfg(target_os = "linux")]
use error::load_env_var;
use error::WallpaperError;

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
#[derive(Debug, Clone, Copy, Educe, EnumString, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
#[educe(Default)]
pub enum Mode {
	///center image witout zooming. Image is may not full visible. Empty space is filled with black.
	Center,
	///zoom image to match x and y size of display and keep aspect ratio. Some parts of the image is may cut off.
	#[educe(Default)]
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
			Self::MacOS => false,
			Self::Windows => false,
			Self::X11 => true,
		}
	}
}

/// include information about a connected screen
#[derive(Clone, Debug)]
pub struct Screen {
	pub name: String,
	wallpaper: Option<String>,
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

	///Set background to wallpapers, witch will selected by the given closure.
	///The index oft screen and the current screen are passed to the closure.
	pub fn set_wallapers<F>(mut self, f: F) -> Result<(), WallpaperError>
	where
		F: Fn(usize, &Screen) -> (String, Mode),
	{
		for (i, screen) in self.screens.iter_mut().enumerate() {
			let tupple = f(i, &screen);
			screen.wallpaper = Some(tupple.0);
			screen.mode = Some(tupple.1)
		}
		set_screens_from_builder(self)
	}

	///Set the background of all screens to the wallpapers of `wallpapers`.
	///The wallpaper of `screen[i]` will be set to `wallpapers[i mod wallpapers.len()]`
	pub fn set_wallpapers_from_vec<T>(self, wallpapers: Vec<T>, mode: Mode) -> Result<(), WallpaperError>
	where
		T: Into<String>,
		T: Clone,
	{
		self.set_wallapers(|i, _| (wallpapers[i % wallpapers.len()].clone().into(), mode))
	}

	///Like [`Self::set_wallpapers_from_vec`],
	///but map the wallpapers randomly to the screens.
	///Selecting the same wallpaper multiple time will be avoid, if this is possible.
	#[cfg(feature = "rand")]
	pub fn set_random_wallpapers_from_vec<T>(self, wallpapers: Vec<T>, mode: Mode) -> Result<(), WallpaperError>
	where
		T: Into<String>,
		T: Clone,
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

pub fn set_wallpapers_from_vec<T>(wallpapers: Vec<T>, mode: Mode) -> Result<(), WallpaperError>
where
	T: Into<String>,
	T: Clone,
{
	let builder = WallpaperBuilder::new()?;
	builder.set_wallpapers_from_vec(wallpapers, mode)
}

#[cfg(feature = "rand")]
pub fn set_random_wallpapers_from_vec<T>(wallpapers: Vec<T>, mode: Mode) -> Result<(), WallpaperError>
where
	T: Into<String>,
	T: Clone,
{
	let builder = WallpaperBuilder::new()?;
	builder.set_wallpapers_from_vec(wallpapers, mode)
}
