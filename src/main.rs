#![windows_subsystem = "windows"]

use tray_item::{IconSource, TrayItem};

enum Message {
    Quit,
}

fn main() {
    let mut tray = TrayItem::new(
        "Tray Example",
        IconSource::Resource("assets/favicon.ico"),
    ).unwrap();
}
