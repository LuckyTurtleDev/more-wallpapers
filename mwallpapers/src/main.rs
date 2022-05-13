use more_wallpapers::{Mode, WallpaperBuilder};

fn main() {
	let builder = WallpaperBuilder::new();
	builder.set_wallapers(|i, len, screen| ("/usr/share/wallpapers/Altai/contents/images/5120x2880.png".into(), Mode::Crop))
}
