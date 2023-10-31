# more-wallpapers ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![more-wallpapers on crates.io](https://img.shields.io/crates/v/more-wallpapers)](https://crates.io/crates/more-wallpapers) [![more-wallpapers on docs.rs](https://docs.rs/more-wallpapers/badge.svg)](https://docs.rs/more-wallpapers) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/more-wallpapers) ![Rust Version: 1.62.0](https://img.shields.io/badge/rustc-1.62.0-orange.svg)

Yet another wallpaper crate, which can set a wallpapers per screen.

The main feature over other crates like [wallpaper][__link0] or [wall][__link1] is the ability to set **different wallpapers** on different screens. Currently this feature is only implemented for some environments. Because of this you can enable the `fallback` feature, to use a [custom version][__link2] of the [wallpaper][__link3] crate as a fallback on unsupported environments. This means you can use the additonal features of this crate and still support a large amount of environments.

Currently the following environments are supported:

| environment | set wallpaper | set wallpaper per screen | requirements |
| --- |:---:|:---:| --- |
| Windows | ✅ | ❌ | `features=["fallback"]`¹ |
| MacOS | ✅ | ❌ | `features=["fallback"]`¹ |
| X11³ | ✅ | ✅ | [xwallpaper][__link4], [libxrandr][__link5]² |
| Budgie(wayland) | ✅ | ❌ | `features=["fallback"]`¹ |
| Cinnamon⁴ | ✅ | ✅ | [xwallpaper][__link6], [libxrandr][__link7]² |
| Deepin(wayland) | ✅ | ❌ | `features=["fallback"]`¹ |
| GNOME(wayland) | ✅ | ❌ | `features=["fallback"]`¹ |
| KDE | ✅ | ✅ |  |
| Mate(wayland) | ✅ | ❌ | `features=["fallback"]`¹ |
| Sway | ✅ | ✅ |  |

¹ Please check also the requirements of the [wallpaper][__link8] crate.<br/> ² Normally already installed.<br/> ³ Wallpapers will be reset after restart. <br/> ⁴ Wallpapers will be reset to provided default after restart.

The information about the currently supported features are also provided by the [`Environment`][__link9] enum.


### QuickStart / Examples:

If you would like to set only a different wallpaper for each screen and don’t care which wallpaper is used on which screen, you can use [`set_wallpapers_from_vec()`][__link10] or [`set_random_wallpapers_from_vec()`][__link11] (only aviable with the `rand` feature):


```rust
use more_wallpapers::Mode;

let images = vec!["1.jpg", "/usr/share/wallpapers/2.jpg"];
more_wallpapers::set_wallpapers_from_vec(images, "default.jpg", Mode::Crop)?;
```

The `"default.jpg"` is used as wallpaper for [inactive screens][__link12]. If you do not know witch value you shoud use here, you can simple use the first elment of the images vec.

For advanced wallpaper settings you can use the [`WallpaperBuilder`][__link13]:


```rust
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
```



 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEG_W_Gn_kaocAGwCcVPfenh7eGy6gYLEwyIe4G6-xw_FwcbpjYXKEG7WNg5vbQg0-G2Kkqe08eywGG2qR1qNCmVseG02ad2s_-Wv5YWSBg29tb3JlLXdhbGxwYXBlcnNlMC4zLjBvbW9yZV93YWxscGFwZXJz
 [__link0]: https://crates.io/crates/wallpaper
 [__link1]: https://crates.io/crates/wall
 [__link10]: https://docs.rs/more-wallpapers/0.3.0/more_wallpapers/?search=set_wallpapers_from_vec
 [__link11]: https://docs.rs/more-wallpapers/0.3.0/more_wallpapers/?search=set_random_wallpapers_from_vec
 [__link12]: https://docs.rs/more-wallpapers/0.3.0/more_wallpapers/?search=Screen::active
 [__link13]: https://docs.rs/more-wallpapers/0.3.0/more_wallpapers/struct.WallpaperBuilder.html
 [__link2]: https://github.com/LuckyTurtleDev/wallpaper.rs
 [__link3]: https://crates.io/crates/wallpaper
 [__link4]: https://github.com/stoeckmann/xwallpaper
 [__link5]: https://gitlab.freedesktop.org/xorg/app/xrandr
 [__link6]: https://github.com/stoeckmann/xwallpaper
 [__link7]: https://gitlab.freedesktop.org/xorg/app/xrandr
 [__link8]: https://crates.io/crates/wallpaper
 [__link9]: https://docs.rs/more-wallpapers/0.3.0/more_wallpapers/enum.Environment.html
