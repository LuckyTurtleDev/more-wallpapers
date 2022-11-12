# more-wallpapers ![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue) [![more-wallpapers on crates.io](https://img.shields.io/crates/v/more-wallpapers)](https://crates.io/crates/more-wallpapers) [![more-wallpapers on docs.rs](https://docs.rs/more-wallpapers/badge.svg)](https://docs.rs/more-wallpapers) [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/LuckyTurtleDev/more-wallpapers) ![Rust Version: ^1.62](https://img.shields.io/badge/rustc-%5E1.62-orange.svg)

Yet another wallpaper crate, which can set a wallpapers per screen.

The main feature over other crates like [wallpaper][__link0] or [wall][__link1] is the ability to set **different wallpapers** on different screens. Currently this feature is only implemented for some enviroments. Because of this you can enable the “wallpaper” feature, which uses the [wallpaper][__link2] crate as a fallback on unsupported environments. This means you can use the additonal features of this crate and still support the large amount of supported enviroments of the [wallpaper][__link3] crate.

Currently the following enviroments are supported:

| enviroment | set wallpaper | set wallpaper per screen | requirements |
| --- |:---:|:---:| --- |
| Windows | ✅ | ❌ | `features=["wallpaper"]`¹ |
| MacOS | ✅ | ❌ | `features=["wallpaper"]`¹ |
| X11 | ✅ | ✅ | [xwallpaper][__link4], [libxrandr][__link5]² |
| Budgie(wayland) | ✅ | ❌ | `features=["wallpaper"]`¹ |
| Deepin(wayland) | ✅ | ❌ | `features=["wallpaper"]`¹ |
| GNOME(wayland) | ✅ | ❌ | `features=["wallpaper"]`¹ |
| KDE | ✅ | ✅ | [dbus][__link6]² |
| Mate(wayland) | ✅ | ❌ | `features=["wallpaper"]`¹ |
| Sway | ✅ | ❌ | `features=["wallpaper"]`¹ |
| some other wayland desktops | ✅ | ❌ | `features=["wallpaper"]`¹, [swaybg][__link7], dektop must support wlr-layer-shell protocol and wl_output version 4 |

¹ Please check also the requirements of the [wallpaper][__link8] crate.</br> ² normally already installed.

The information about the currently supported features are also provided by the [`Enviroment`][__link9] enum.


#### QuickStart / Examples:

If you would like to set only a different wallpaper for each screen and don’t care which wallpaper is used on which screen, you can use [`set_wallpapers_from_vec()`][__link10] or [`set_random_wallpapers_from_vec()`][__link11]:


```rust
use more_wallpapers::Mode;

let images = vec!["1.jpg", "/usr/share/wallpapers/2.jpg"];
more_wallpapers::set_wallpapers_from_vec(images, Mode::Crop)?;
```

For advanced wallpaper settings you can use the [`WallpaperBuilder`][__link12]:


```rust
use more_wallpapers::{Mode, WallpaperBuilder};

let fallback_images = vec!["/usr/share/wallpapers/1.jpg", "/usr/share/wallpapers/2.jpg"];
WallpaperBuilder::new()?.set_wallapers(|i, screen| -> (String, Mode) {
	if i == 0 {
		return (
			"first.jpg".to_owned(),
			Mode::default(),
		);
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


 [__cargo_doc2readme_dependencies_info]: ggGkYW0BYXSEGyDwipHVMb5RGxgd3zutc1TvG3ARKV4UcQ1NGyM1aXabIPYbYXKEG8OS-vvjt7NWG4TO9g3EvN1bG_vYFzucHWl_G1HCoq8kIOd-YWSBg29tb3JlLXdhbGxwYXBlcnNlMC4xLjFvbW9yZV93YWxscGFwZXJz
 [__link0]: https://crates.io/crates/wallpaper
 [__link1]: https://crates.io/crates/wall
 [__link10]: https://docs.rs/more-wallpapers/0.1.1/more_wallpapers/?search=set_wallpapers_from_vec
 [__link11]: https://docs.rs/more-wallpapers/0.1.1/more_wallpapers/?search=set_random_wallpapers_from_vec
 [__link12]: https://docs.rs/more-wallpapers/0.1.1/more_wallpapers/struct.WallpaperBuilder.html
 [__link2]: https://crates.io/crates/wallpaper
 [__link3]: https://crates.io/crates/wallpaper
 [__link4]: https://github.com/stoeckmann/xwallpaper
 [__link5]: https://gitlab.freedesktop.org/xorg/app/xrandr
 [__link6]: https://gitlab.freedesktop.org/dbus/dbus
 [__link7]: https://github.com/swaywm/swaybg
 [__link8]: https://crates.io/crates/wallpaper
 [__link9]: https://docs.rs/more-wallpapers/0.1.1/more_wallpapers/enum.Enviroment.html
