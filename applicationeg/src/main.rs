use std::ffi::OsStr;
use std::io;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use winapi::shared::windef::HICON;
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::{DestroyIcon, LoadImageW};
use winreg::enums::*;
use winreg::RegKey;

// 定义一个结构体来保存图标句柄和路径等信息
pub struct IconInfo {
    pub icon_handle: HICON,
    pub icon_path: String,
}

#[derive(Debug)]
struct InstalledSoftware {
    name: String,
    icon: Option<String>,
    install_location: Option<String>,
}

fn list_installed_software() -> io::Result<Vec<InstalledSoftware>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_key = hklm.open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        KEY_READ,
    )?;

    let mut software_list = Vec::new();

    for entry in uninstall_key.enum_keys()? {
        let entry_key = uninstall_key.open_subkey_with_flags(&entry, KEY_READ)?;

        let display_name: Option<String> = entry_key.get_value("DisplayName")?;
        let display_icon: Option<String> = entry_key.get_value("DisplayIcon")?;
        let install_location: Option<String> = entry_key.get_value("InstallLocation")?;

        if let Some(name) = display_name {
            software_list.push(InstalledSoftware {
                name,
                icon: display_icon,
                install_location,
            });
        }
    }

    Ok(software_list)
}

// 提取图标函数
fn extract_icon_from_path(icon_path: &str, resource_index: u32) -> Option<IconInfo> {
    unsafe {
        // 将字符串转换为宽字符
        let path = OsStr::new(icon_path)
            .encode_wide()
            .chain(once(0))
            .collect::<Vec<u16>>();
        let module_handle = GetModuleHandleW(path.as_ptr());

        if module_handle.is_null() {
            return None;
        }

        // 使用LoadImageW加载图标
        let icon_handle = LoadImageW(
            module_handle,
            resource_index as *const _,
            winapi::um::winuser::IMAGE_ICON,
            0, // 使用系统默认大小
            0, // 使用系统默认大小
            winapi::um::winuser::LR_DEFAULTSIZE | winapi::um::winuser::LR_LOADFROMFILE,
        ) as HICON;

        if icon_handle.is_null() {
            return None;
        }

        Some(IconInfo {
            icon_handle,
            icon_path: icon_path.to_string(),
        })
    }
}

fn main() {
    let icon_path = r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe";
    let resource_index = 0; // 假设资源索引为0

    if let Some(icon_info) = extract_icon_from_path(icon_path, resource_index) {
        println!("Icon extracted from: {}", icon_info.icon_path);

        // 在使用完图标后需要销毁图标
        unsafe {
            DestroyIcon(icon_info.icon_handle);
        }
    } else {
        println!("Failed to extract icon");
    }
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
