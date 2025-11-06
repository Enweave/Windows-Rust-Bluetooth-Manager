#![windows_subsystem = "windows"]

use image::GenericImageView;
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use windows::{
    core::Result,
    Devices::Radios::{Radio, RadioState},
};
use tokio::runtime::Builder;
use toml::Value;

fn main() {
    let cargo_toml_str = include_str!("../Cargo.toml");
    let cargo_toml: Value = toml::from_str(cargo_toml_str).unwrap();
    let version: String = cargo_toml["package"]["version"].as_str().unwrap().to_owned();
    let repository: String = cargo_toml["package"]["repository"].as_str().unwrap().to_owned();

    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let icon_bytes = include_bytes!("../assets/favicon.ico");
    let image = image::load_from_memory(icon_bytes).unwrap();
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8().into_raw();
    let color_icon = Icon::from_rgba(rgba, width, height).unwrap();

    let grayscale_rgba = image.grayscale().to_rgba8().into_raw();
    let gray_icon = Icon::from_rgba(grayscale_rgba, width, height).unwrap();

    let initial_state = rt.block_on(async { get_bluetooth_state().await.ok().flatten() });
    let toggle_text = get_toggle_text(initial_state);
    let initial_icon = if initial_state == Some(RadioState::On) {
        color_icon.clone()
    } else {
        gray_icon.clone()
    };

    let event_loop = EventLoopBuilder::new().build();

    let tray_menu = Menu::new();
    let toggle_item = MenuItem::new(toggle_text, true, None);
    let version_item = MenuItem::new(format!("Version {}", version), true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append_items(&[&toggle_item, &version_item, &quit_item]).unwrap();

    let tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Bluetooth Manager")
        .with_icon(initial_icon)
        .build()
        .unwrap();

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_item.id() {
                *control_flow = ControlFlow::Exit;
            } else if event.id == toggle_item.id() {
                rt.block_on(async {
                    if let Err(e) = toggle_bluetooth().await {
                        eprintln!("Error toggling Bluetooth: {:?}", e);
                    }
                    let new_state = get_bluetooth_state().await.ok().flatten();
                    toggle_item.set_text(get_toggle_text(new_state));
                    if new_state == Some(RadioState::On) {
                        tray_icon.set_icon(Some(color_icon.clone())).unwrap();
                    } else {
                        tray_icon.set_icon(Some(gray_icon.clone())).unwrap();
                    }
                });
            } else if event.id == version_item.id() {
                webbrowser::open(&repository).unwrap();
            }
            println!("{event:?}");
        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    });
}

async fn toggle_bluetooth() -> Result<()> {
    let radios = Radio::GetRadiosAsync()?.await?;
    for radio in radios {
        if radio.Kind()? == windows::Devices::Radios::RadioKind::Bluetooth {
            let state = radio.State()?;
            if state == RadioState::On {
                radio.SetStateAsync(RadioState::Off)?.await?;
            } else {
                radio.SetStateAsync(RadioState::On)?.await?;
            }
        }
    }
    Ok(())
}

async fn get_bluetooth_state() -> Result<Option<RadioState>> {
    let radios = Radio::GetRadiosAsync()?.await?;
    for radio in radios {
        if radio.Kind()? == windows::Devices::Radios::RadioKind::Bluetooth {
            return Ok(Some(radio.State()?));
        }
    }
    Ok(None)
}

fn get_toggle_text(state: Option<RadioState>) -> &'static str {
    if let Some(RadioState::On) = state {
        "Toggle Bluetooth Off"
    } else {
        "Toggle Bluetooth On"
    }
}
