extern crate winreg;

use std::io;
use winreg::enums::*;
use winreg::RegKey;
use std::fs;
use std::path::Path;

fn main() -> io::Result<()> {
    // 打开注册表中的 Uninstall 路径
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // 定义需要检查的注册表路径
    let uninstall_paths = vec![
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    ];

    // 遍历注册表路径
    for uninstall_path in uninstall_paths {
        if let Ok(key) = hklm.open_subkey(uninstall_path) {
            // list_install_locations(&key)?;
            list_all_subkey_fields(&key)?;
        }

        if let Ok(key) = hkcu.open_subkey(uninstall_path) {
            // list_install_locations(&key)?;
            list_all_subkey_fields(&key)?;
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn list_install_locations(key: &RegKey) -> io::Result<()> {
    // 遍历注册表中的子键
    for subkey_name in key.enum_keys().filter_map(|x| x.ok()) {
        if let Ok(subkey) = key.open_subkey(&subkey_name) {
            // 读取 DisplayName 和 InstallLocation
            if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                if let Ok(install_location) = subkey.get_value::<String, _>("InstallLocation") {
                    if !install_location.is_empty() {
                        println!("软件名称: {}", display_name);
                        println!("安装路径: {}", install_location);
                        println!("-----------------------------");
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn list_all_subkey_fields(key: &RegKey) -> io::Result<()> {
    // 遍历注册表中的子键
    for subkey_name in key.enum_keys().filter_map(|x| x.ok()) {
        if let Ok(subkey) = key.open_subkey(&subkey_name) {
            println!("软件子键: {}", subkey_name);
            println!("-----------------------------");

            // 遍历子键中的所有字段
            for value_name in subkey.enum_values().filter_map(|x| x.ok()) {
                let value_data = match subkey.get_raw_value(&value_name) {
                    Ok(data) => format!("{:?}", data),
                    Err(_) => "无法读取".to_string(),
                };
                println!("{}: {}", value_name, value_data);
            }

            println!("-----------------------------");
        }
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn list_installed_software_mac() {
    let applications_dir = Path::new("/Applications");
    if applications_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(applications_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() && path.extension().map_or(false, |ext| ext == "app") {
                        println!("Application: {}", path.display());
                    }
                }
            }
        }
    }
}