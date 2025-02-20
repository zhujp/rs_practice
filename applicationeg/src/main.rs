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


// 定义应用程序信息结构体
#[derive(Debug)]
struct AppInfo {
    name: String,
    icon_path: Option<PathBuf>,
}
///System/Applications/Calendar.app  //系统应用
fn list_applications() -> Result<Vec<AppInfo>, std::io::Error> {
    let applications_dir = Path::new("/Applications");
    let mut apps = Vec::new();

    if applications_dir.is_dir() {
        for entry in fs::read_dir(applications_dir)? {
            let entry = entry?;
            let path = entry.path();

            // 检查是否是目录且以 .app 结尾
            if path.is_dir() && path.extension().map_or(false, |ext| ext == "app") {
                let app_name = path.file_name().unwrap().to_string_lossy().to_string();

                // 查找图标文件
                let icon_path = find_icon_in_app_bundle(&path);

                apps.push(AppInfo {
                    name: app_name,
                    icon_path,
                });
            }
        }
    } else {
        eprintln!("'/Applications' directory not found.");
    }

    Ok(apps)
}

// 查找应用程序包中的图标文件
fn find_icon_in_app_bundle(app_path: &Path) -> Option<PathBuf> {
    let resources_path = app_path.join("Contents/Resources");

    if resources_path.is_dir() {
        // 遍历 Resources 目录，查找 .icns 文件
        if let Ok(entries) = fs::read_dir(resources_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "icns") {
                    return Some(path);
                }
            }
        }
    }

    None
}

fn open_application(app_name: &str) -> Result<(), std::io::Error> {
    let status = Command::new("open")
        .arg("-a")
        .arg(app_name)
        .status()?;

    if status.success() {
        println!("Successfully opened: {}", app_name);
    } else {
        eprintln!("Failed to open: {}", app_name);
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    // 列出所有应用程序及其图标
    let apps = list_applications()?;
    println!("Installed Applications:");
    for app in &apps {
        println!("- {}", app.name);
        if let Some(icon_path) = &app.icon_path {
            println!("  Icon: {}", icon_path.display());
        } else {
            println!("  Icon: Not found");
        }
    }

    // 提示用户输入应用程序名称
    print!("Enter the name of the application to open: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let app_name = input.trim();

    // 检查输入的应用程序是否存在
    if apps.iter().any(|app| app.name == app_name) {
        open_application(app_name)?;
    } else {
        eprintln!("Application '{}' not found.", app_name);
    }

    Ok(())
}