use crate::{Mode, Screen};
use dbus::blocking::Connection;
use std::time::Duration;

pub(crate) fn get_screens() -> Vec<Screen> {
	let destination = "org.kde.plasmashell";
	let interface = "org.kde.PlasmaShell";
	let path = "/PlasmaShell";
	let method = "evaluateScript";
	let args = ("print(\"hhi\")",);
	let timeout = Duration::from_millis(5000);
	let conn = Connection::new_session().unwrap();
	let proxy = conn.with_proxy(destination, path, timeout);
	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
	let (names,): (String,) = proxy.method_call(interface, method, args).unwrap();
	vec![]
}

pub(crate) fn set_screens(screens: Vec<Screen>) {}
