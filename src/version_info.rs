use toml::Value;
use tray_icon::menu::MenuItem;

pub struct VersionInfo {
    pub item: MenuItem,
    pub repository: String,
}

pub fn create_version_item() -> VersionInfo {
    let cargo_toml_str = include_str!("../Cargo.toml");
    let cargo_toml: Value = toml::from_str(cargo_toml_str).unwrap();
    let version: String = cargo_toml["package"]["version"]
        .as_str()
        .unwrap()
        .to_owned();
    let repository: String = cargo_toml["package"]["repository"]
        .as_str()
        .unwrap()
        .to_owned();

    let item = MenuItem::new(format!("Version {}", version), true, None);

    VersionInfo { item, repository }
}

