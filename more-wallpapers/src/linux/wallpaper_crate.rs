use crate::Screen;

pub(crate) fn get_screens() -> Vec<Screen> {
	vec![Screen {
		name: "Unknow".into(),
		active: true,
		wallpaper: None,
		mode: None,
	}]
}

pub(crate) fn set_screens(screen: Vec<Screen>) -> Result<(), fallback::Error> {
	let screen = screen.first().unwrap();
	fallback::set_from_path(screen.wallpaper.as_ref().unwrap().as_str())?;
	fallback::set_mode(screen.mode.unwrap().into())
}
