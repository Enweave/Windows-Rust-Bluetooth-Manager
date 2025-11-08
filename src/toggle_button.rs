// src/toggle_button.rs
use tray_icon::{
    menu::MenuItem,
    Icon, TrayIcon,
};
use windows::{
    core::Result,
    Devices::Radios::{Radio, RadioState},
};

pub struct ToggleButton {
    pub item: MenuItem,
}

impl ToggleButton {
    pub fn new(initial_state: Option<RadioState>) -> Self {
        let toggle_text = get_toggle_text(initial_state);
        let item = MenuItem::new(toggle_text, true, None);
        Self { item }
    }

    pub async fn handle_click(
        &self,
        tray_icon: &TrayIcon,
        color_icon: &Icon,
        gray_icon: &Icon,
    ) {
        if let Err(e) = toggle_bluetooth().await {
            eprintln!("Error toggling Bluetooth: {:?}", e);
        }
        let new_state = get_bluetooth_state().await.ok().flatten();
        self.item.set_text(get_toggle_text(new_state));
        if new_state == Some(RadioState::On) {
            tray_icon.set_icon(Some(color_icon.clone())).unwrap();
        } else {
            tray_icon.set_icon(Some(gray_icon.clone())).unwrap();
        }
    }
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

pub async fn get_bluetooth_state() -> Result<Option<RadioState>> {
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
