use tray_icon::menu::{MenuItem};
use windows::core::HSTRING;
use windows::Devices::Bluetooth::BluetoothDevice;
use windows::Devices::Enumeration::{DeviceInformation, DeviceInformationCollection};

pub async fn get_devices(selector: &str) -> windows::core::Result<DeviceInformationCollection> {
    DeviceInformation::FindAllAsyncAqsFilter(&HSTRING::from(selector))?.await
}

pub async fn create_devices_items() -> Vec<MenuItem> {
    let mut devices_menu_items: Vec<MenuItem> = vec![];

    if let Ok(paired_selector) = BluetoothDevice::GetDeviceSelectorFromPairingState(true) {
        if let Ok(paired_devices) = get_devices(&paired_selector.to_string()).await {
            for device_info in paired_devices {
                let mut name = device_info.Name().unwrap_or_else(|_| "Unknown".into()).to_string();
                if let Ok(id) = device_info.Id() {
                    if let Ok(device_op) = BluetoothDevice::FromIdAsync(&id) {
                        if let Ok(device) = device_op.await {
                            if let Ok(status) = device.ConnectionStatus() {
                                if status == windows::Devices::Bluetooth::BluetoothConnectionStatus::Connected {
                                    name.push_str(" (Connected)");
                                }
                            }
                        }
                    }
                }
                devices_menu_items.push(MenuItem::new(name, true, None));
            }
        }
    }

    if let Ok(discovered_selector) = BluetoothDevice::GetDeviceSelectorFromPairingState(false) {
        if let Ok(discovered_devices) = get_devices(&discovered_selector.to_string()).await {
            if discovered_devices.Size().unwrap_or(0) > 0 {
                for device_info in discovered_devices {
                    let name = device_info.Name().unwrap_or_else(|_| "Unknown".into()).to_string();
                    devices_menu_items.push(MenuItem::new(name, true, None));
                }
            }
        }
    }

    devices_menu_items
}

