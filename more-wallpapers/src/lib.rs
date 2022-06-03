#![warn(unreachable_pub)]
#![cfg_attr(all(doc, nightly), feature(doc_auto_cfg))]

use educe::Educe;
use strum_macros::Display;

#[cfg(feature = "rand")]
use rand::{prelude::IteratorRandom, seq::SliceRandom};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use crate::linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use crate::windows::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use crate::macos::*;

#[derive(Debug, Clone, Copy, Educe, Display, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Enviroment {
	Kde,
	LinuxWallpaperCrate,
	MacOS,
	Windows,
	X11,
}
impl Enviroment {
	///return true, if the current enviroment does support various wallpaper on each screen
	pub fn support_various_wallpaper(&self) -> bool {
		match self {
			Self::Kde => true,
			Self::LinuxWallpaperCrate => false,
			Self::MacOS => false,
			Self::Windows => false,
			Self::X11 => true,
		}
	}
}

#[derive(Clone, Debug)]
pub struct Screen {
	pub name: String,
	wallpaper: Option<String>,
	mode: Option<Mode>,
}

///Builder for advance Wallpaper settings.
///This struct should not be stored for a long time, because it can become outdated if the user connect or disconnect monitors or change the Display settings.
#[derive(Clone, Debug)]
pub struct WallpaperBuilder {
	screens: Vec<Screen>,
	enviroment: Enviroment,
}

impl WallpaperBuilder {
	pub fn new() -> Self {
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
	pub fn set_wallapers<F>(mut self, f: F)
	where
		F: Fn(usize, &Screen) -> (String, Mode),
	{
		for (i, screen) in self.screens.iter_mut().enumerate() {
			let tupple = f(i, &screen);
			screen.wallpaper = Some(tupple.0);
			screen.mode = Some(tupple.1)
		}
		set_screens_from_builder(self);
	}

	///Set the background of all screens to the wallpapers of `wallpapers`.
	///The wallpaper of `screen[i]` will be set to `wallpapers[i mod wallpapers.len()]`
	pub fn set_wallpapers_from_vec<T>(self, wallpapers: Vec<T>, mode: Mode)
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
	pub fn set_random_wallpapers_from_vec<T>(self, wallpapers: Vec<T>, mode: Mode)
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

pub fn set_wallpapers_from_vec<T>(wallpapers: Vec<T>, mode: Mode)
where
	T: Into<String>,
	T: Clone,
{
	let builder = WallpaperBuilder::new();
	builder.set_wallpapers_from_vec(wallpapers, mode);
}

#[cfg(feature = "rand")]
pub fn set_random_wallpapers_from_vec<T>(wallpapers: Vec<T>, mode: Mode)
where
	T: Into<String>,
	T: Clone,
{
	let builder = WallpaperBuilder::new();
	builder.set_wallpapers_from_vec(wallpapers, mode);
}
