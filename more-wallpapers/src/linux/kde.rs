use crate::{error::WallpaperError, Mode, Screen};
use rustbus::{
	connection,
	connection::{ll_conn::force_finish_on_error, Timeout},
	MessageBuilder, RpcConn,
};
use serde::Deserialize;
use std::{fmt::Write as _, time::Duration};

#[derive(Deserialize)]
struct KdeDesktop {
	screen: i32,
	id: u32,
}

fn plasmashell(command: &str) -> Result<String, connection::Error> {
	let session_path = rustbus::get_session_bus_path()?;
	let mut con = RpcConn::connect_to_path(session_path, Timeout::Duration(Duration::from_millis(5000)))?;
	let mut call = MessageBuilder::new()
		.call("evaluateScript")
		.with_interface("org.kde.PlasmaShell")
		.on("/PlasmaShell")
		.at("org.kde.plasmashell")
		.build();
	call.body.push_param(command)?;
	let id = con.send_message(&mut call)?.write_all().map_err(force_finish_on_error)?;
	let message = con.wait_response(id, Timeout::Duration(Duration::from_millis(5000)))?;
	Ok(message.body.parser().get::<&str>().unwrap().to_owned())
}

pub(crate) fn get_screens() -> Result<Vec<Screen>, WallpaperError> {
	let desktops: Vec<KdeDesktop> = serde_json::from_str(&plasmashell("print(JSON.stringify(desktops()));")?)?;
	let mut screens = std::vec::Vec::new();
	for desktop in desktops {
		if desktop.screen >= 0 {
			screens.push(Screen {
				name: desktop.id.to_string(),
				wallpaper: None,
				mode: None,
			});
		}
	}
	Ok(screens)
}

pub(crate) fn set_screens(screens: Vec<Screen>) -> Result<(), connection::Error> {
	let mut command = r#"
	for (const desktop of desktops()) {
		desktop.currentConfigGroup = ["Wallpaper", "org.kde.image", "General"];"#
		.to_owned();
	for screen in screens {
		let mode = match screen.mode.unwrap() {
			Mode::Center => 6,
			Mode::Crop => 2,
			Mode::Fit => 1,
			Mode::Stretch => 0,
			Mode::Tile => 3,
		};
		write!(
			command,
			r#"
		if (desktop.id === {}){{
			desktop.writeConfig("FillMode", {});
			desktop.writeConfig("Image", {:?});
		}}"#,
			screen.name,
			mode,
			screen.wallpaper.as_ref().unwrap().as_str()
		)
		.unwrap();
	}
	command += r#"
	}"#;
	plasmashell(&command)?;
	Ok(())
}
