#![windows_subsystem = "windows"]

use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder};
use winit::event_loop::{ControlFlow, EventLoopBuilder};

enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

fn main() {
    let icon = Icon::from_path("../assets/favicon.ico", Some((32u32, 32u32))).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()
        .unwrap();

    let quit_item = MenuItem::new("Quit", true, None);

    let tray_menu = Menu::new();
    tray_menu.append_items(&[&quit_item]).unwrap();



}
