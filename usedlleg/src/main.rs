use libc::{c_int, c_void};
use std::ffi::{CString, OsStr};
use std::iter::once;
use std::mem::transmute;
use std::os::windows::ffi::OsStrExt;

// 定义DEVNOTIFY类型
type DEVNOTIFY = extern "C" fn(*mut c_void, c_int, c_int, c_int);

// 定义DEVPTR类型
type DEVPTR = *mut c_void;

// 回调函数实现
extern "C" fn notify_callback(dev: *mut c_void, msg: c_int, param1: c_int, param2: c_int) {
    println!(
        "Callback called with msg: {}, param1: {}, param2: {}",
        msg, param1, param2
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载dll
    let dll = unsafe { libloading::Library::new("KBAccess.dll") }?;

    // 获取OpenDevice函数
    let open_device: libloading::Symbol<
        unsafe extern "C" fn(*const u16, *mut c_void, Option<DEVNOTIFY>, *mut c_int) -> DEVPTR,
    > = unsafe { dll.get(b"OpenDevice\0")? };

    let get_dll_version: libloading::Symbol<unsafe extern "C" fn() -> c_int> =
        unsafe { dll.get(b"GetDLLVersion\0")? };

    let close_device: libloading::Symbol<unsafe extern "C" fn(DEVPTR) -> c_int> =
        unsafe { dll.get(b"CloseDevice\0")? };

    let device_online: libloading::Symbol<unsafe extern "C" fn(DEVPTR) -> c_int> =
        unsafe { dll.get(b"IsDeviceOnline\0")? };

    let read_battery: libloading::Symbol<
        unsafe extern "C" fn(DEVPTR, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    > = unsafe { dll.get(b"ReadBattery\0")? };

    let new_key_action: libloading::Symbol<
        unsafe extern "C" fn(c_int, c_int, c_int, *mut u8, c_int) -> c_int,
    > = unsafe { dll.get(b"new_key_action\0")? };

    // 将参数转换为宽字符
    let param_str = OsStr::new("57d0c10254559e4a550c00000004dd")
        .encode_wide()
        .chain(once(0))
        .collect::<Vec<u16>>();

    // 创建一个错误码变量
    let mut err_code: c_int = 0;

    // 调用OpenDevice函数
    let dev_ptr = unsafe {
        open_device(
            param_str.as_ptr(),
            std::ptr::null_mut(), // 这里假设hWindowWnd为NULL
            Some(notify_callback),
            &mut err_code as *mut c_int,
        )
    };

    let dll_version = unsafe { get_dll_version() };

    let is_device_online = unsafe { device_online(dev_ptr) };

    let mut battery: c_int = 0;
    let mut batter_charging: c_int = 0;
    let mut battery_full: c_int = 0;

    unsafe {
        read_battery(
            dev_ptr,
            &mut battery,
            &mut batter_charging,
            &mut battery_full,
        );
    };
    // let key_data: *mut u8 = std::ptr::null_mut();
    let mut key_data: Vec<u8> = vec![0; 6];
    let is_succ = unsafe { new_key_action(69, 1, 496, key_data.as_mut_ptr(), 6) };

    unsafe {
        // 调用CloseDevice函数
        close_device(dev_ptr);
    };

    // if !key_data.is_null() {
    //     unsafe {
    //         println!("new key action:is_succ:{}, value:{}", is_succ, *key_data);
    //         // 记得在适当的时候释放key_data指向的内存，避免内存泄漏
    //     }
    // } else {
    //     println!("new key action failed to allocate memory for key_data");
    // }
    println!("{:?}", key_data);
    if dev_ptr.is_null() {
        println!("Failed to open device, error code: {}", err_code);
    } else {
        println!("devptr:{:?}", dev_ptr);
        println!("DLL version: {}", dll_version);
        println!("Device online: {}", is_device_online);
        println!(
            "Battery level: {},{},{}",
            battery, batter_charging, battery_full
        );

        println!("Device opened successfully");
    }

    Ok(())
}
