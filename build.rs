extern crate winres;
use std::fs;
use toml::Value;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/favicon.ico");

        let cargo_toml_string = fs::read_to_string("Cargo.toml").unwrap();
        let cargo_toml: Value = toml::from_str(&cargo_toml_string).unwrap();
        if let Some(package) = cargo_toml.get("package") {
            if let Some(repo) = package.get("repository") {
                if let Some(repo_str) = repo.as_str() {
                    res.set("LegalCopyright", repo_str);
                }
            }
        }

        res.compile().unwrap();
    }
}
