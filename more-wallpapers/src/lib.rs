#![warn(unreachable_pub)]
use educe::Educe;
use rand::{prelude::IteratorRandom, seq::SliceRandom};

#[cfg(all(unix, not(target_os = "macos")))]
mod linux;
#[cfg(all(unix, not(target_os = "macos")))]
use crate::linux::*;

mod wallpaper_crate;

#[derive(Clone, Debug, Educe)]
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

#[derive(Clone, Debug)]
pub enum Enviroment {
	X11,
	WALLPAPER_CRATE,
}

#[derive(Clone, Debug)]
pub struct Screen {
	pub name: String,
	wallpaper: Option<String>,
	mode: Option<Mode>,
}

#[derive(Clone, Debug)]
pub struct WallpaperBuilder {
	screens: Vec<Screen>,
	enviroment: Enviroment,
}

impl WallpaperBuilder {
	pub fn new() -> Self {
		get_builder()
	}

	pub fn screen_count(&self) -> usize {
		self.screens.len()
	}

	pub fn set_wallapers<F>(mut self, f: F)
	where
		F: Fn(usize, usize, &Screen) -> (String, Mode),
	{
		let len = self.screens.len();
		for (i, screen) in self.screens.iter_mut().enumerate() {
			let tupple = f(i, len, &screen);
			screen.wallpaper = Some(tupple.0);
			screen.mode = Some(tupple.1)
		}
		set_screens_from_builder(self);
	}
}

pub fn set_wallpapers_from_vec<T>(wallpapers: Vec<T>, random: bool)
where
	T: Into<String>,
	T: Clone,
{
	let builder = WallpaperBuilder::new();
	let chosen_wallpapers: Vec<T> = if random {
		let mut rng = rand::thread_rng();
		let wallpapers = if wallpapers.len() < builder.screen_count() {
			//extend vec to match length of screen_count
			let mut new_wallpapers = Vec::new();
			while new_wallpapers.len() < builder.screen_count() {
				let count = (builder.screen_count() - new_wallpapers.len()).min(wallpapers.len());
				let mut add = wallpapers.clone().into_iter().choose_multiple(&mut rng, count);
				new_wallpapers.append(&mut add);
			}
			new_wallpapers
		} else {
			wallpapers
		};
		let mut choose_wallpapers = wallpapers.into_iter().choose_multiple(&mut rng, builder.screen_count());
		choose_wallpapers.shuffle(&mut rng);
		choose_wallpapers
	} else {
		wallpapers
	};
	builder.set_wallapers(|i, _, _| (chosen_wallpapers[i % chosen_wallpapers.len()].clone().into(), Mode::Crop))
}
