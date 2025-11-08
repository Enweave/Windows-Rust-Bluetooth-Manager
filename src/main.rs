// src/main.rs
#![windows_subsystem = "windows"]

use image::GenericImageView;
use tokio::runtime::Builder;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use windows::Devices::Radios::RadioState;
use winit::event_loop::{ControlFlow, EventLoopBuilder};

mod toggle_button;
mod version_info;

fn main() {
    let version_info = version_info::create_version_item();

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let icon_bytes = include_bytes!("../assets/favicon.ico");
    let image = image::load_from_memory(icon_bytes).unwrap();
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8().into_raw();
    let color_icon = Icon::from_rgba(rgba, width, height).unwrap();

    let grayscale_rgba = image.grayscale().to_rgba8().into_raw();
    let gray_icon = Icon::from_rgba(grayscale_rgba, width, height).unwrap();

    let initial_state = rt.block_on(async { toggle_button::get_bluetooth_state().await.ok().flatten() });
    let toggle_button = toggle_button::ToggleButton::new(initial_state);

    let initial_icon = if initial_state == Some(RadioState::On) {
        color_icon.clone()
    } else {
        gray_icon.clone()
    };

    let event_loop = EventLoopBuilder::new().build();

    let tray_menu = Menu::new();
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu
        .append_items(&[&toggle_button.item, &version_info.item, &quit_item])
        .unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Bluetooth Manager")
        .with_icon(initial_icon)
        .build()
        .unwrap();

    let menu_channel = MenuEvent::receiver();
    let _tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_item.id() {
                *control_flow = ControlFlow::Exit;
            } else if event.id == toggle_button.item.id() {
                rt.block_on(async {
                    toggle_button
                        .handle_click(&tray_icon, &color_icon, &gray_icon)
                        .await;
                });
            } else if event.id == version_info.item.id() {
                webbrowser::open(&version_info.repository).unwrap();
            }
        }
    });
}
