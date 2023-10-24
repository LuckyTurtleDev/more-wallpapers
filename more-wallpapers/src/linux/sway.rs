use crate::{
	error::{CommandError, WallpaperError},
	linux::{run, x11},
	Mode, Screen,
};
use std::{process::Command, io::StdoutLock};
use super::check_command_error;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
struct SawyMsgOutput
{
    #[serde(rename(serialize = "all_screens"))]
    all_screens: Vec<Output_Screen>
}
#[derive(Deserialize, Debug)]
pub struct Output_Screen{
    name: String,
    wallpaper: String,
    mode: String,
    active: bool,
    
}

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
    let mut command = Command::new("swaymsg");
    command.args(["-t", "get_outputs"]);
    let output = check_command_error(command.output(), "swaymsg")?;
    let output = String::from_utf8(output).unwrap();
    println!("{output}");
    let output: SawyMsgOutput = serde_json::from_str(&output)?;
    println!("{output:#?}");
	todo!()//Ok(screens)
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), WallpaperError> {
	Ok(())
}
