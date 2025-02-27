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
