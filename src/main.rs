#![windows_subsystem = "windows"]

use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIconBuilder, TrayIconEvent};
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use windows::{
    core::Result,
    Devices::Radios::{Radio, RadioState},
};
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/favicon.ico");
    let icon = Icon::from_path(path, Some((32, 32))).unwrap();
    let event_loop = EventLoopBuilder::new().build();

    let tray_menu = Menu::new();
    let toggle_item = MenuItem::new("Toggle Bluetooth", true, None);
    let quit_item = MenuItem::new("Quit", true, None);
    tray_menu.append_items(&[&toggle_item, &quit_item]).unwrap();

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Bluetooth Manager")
        .with_icon(icon)
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
                });
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
