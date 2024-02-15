use crate::{error::CommandError, load_env_var, Environment, WallpaperBuilder, WallpaperError};
use crate::Screen;
use super::check_command_error;
use std::process::Command;

fn load_property(property: &str) -> Result<String, WallpaperError>  {
	let mut command = Command::new("xfconf-query");
			command.args(["--channel", "xfce4-desktop", "p"]);
			command.arg(property);
			let output = check_command_error(command.output(), "xfconf-query")?;
			let output = String::from_utf8(output).unwrap();
			Ok(output)
}


pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	let mut command = Command::new("xfconf-query");
	command.args(["--channel", "xfce4-desktop", "--list"]);
	let output = check_command_error(command.output(), "xfconf-query")?;
	let output = String::from_utf8(output).unwrap();
	let mut current_screen_wallpaper = None; 
	let mut current_screen_mode = None; 
	for line in output.lines() {
		if line.starts_with("/backdrop/") {
			break;
		}
		if line.ends_with("/image-style"){
			let value = load_property(line)?;
			current_screen_mode.is_some panic
			current_screen_mode = Some(("screenname_TODO".to_owned(), value));
			
		}
		if line.ends_with("/last-image"){
			let value = load_property(line)?;
			current_screen_mode.unwrap() = "screenname_TODO";
			current_screen_wallpaper.is_some panic
			current_screen_wallpaper = Some(("screenname_TODO".to_owned(), value));
		}
	}
	todo!()
}