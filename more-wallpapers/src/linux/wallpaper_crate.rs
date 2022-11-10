use crate::Screen;

pub(crate) fn get_screens() -> Vec<Screen> {
	vec![Screen {
		name: "Unknow".into(),
		wallpaper: None,
		mode: None,
	}]
}

pub(crate) fn set_screens(screen: Vec<Screen>) -> Result<(), Box<dyn std::error::Error>> {
	wallpaper::set_from_path(screen[0].wallpaper.as_ref().unwrap().as_str())
}
