use libc::{c_int, c_void};
use libloading::{Library, Symbol};
use std::ffi::{CString, OsStr, OsString};
use std::mem::MaybeUninit;
use std::os::windows::ffi::OsStrExt;
use std::{ptr, slice};

// 定义与C函数对应的类型
type YJXSDK_Init_Type = unsafe extern "C" fn();
type YJXSDK_OpenDevice_Type =
    unsafe extern "C" fn(vid: c_int, pid: c_int, devname: *const u16) -> c_int;

type YJXSDK_FindDevice_Type =
    unsafe extern "C" fn(vid: c_int, pid: c_int, devname: *const u16) -> c_int;

type YJXSDK_DeviceIsOnline_Type = unsafe extern "C" fn(c_int) -> c_int;

type YJXSDK_DeviceVersion_Type = unsafe extern "C" fn(c_int, *mut c_int) -> c_int;

type YJXSDK_GetDeviceBatteryInfo_Type =
    unsafe extern "C" fn(c_int, *mut c_int, *mut c_int) -> c_int;

#[repr(C)]
#[derive(Debug)]
struct YJX_MOUSEINFO {
    profile: u8,               // 板载配置（0 ~ 5）
    work_mode: u8,             // 0: USB, 1: 2.4G, 2: 蓝牙
    is_online: u8,             // 是否在线
    battery_value: u8,         // 电量值
    charge_flag: u8,           // 是否在充电,0: 未充电，1：充电，2：充满
    light_mode: u8,            // 灯光模式
    dpi_count: u8,             // 当前DPI数量, 默认6档
    dpi_index: u8,             // 当前DPI处于那个档位
    dpi1_value: i32,           // 第1档DPI值
    dpi2_value: i32,           // 第2档DPI值
    dpi3_value: i32,           // 第3档DPI值
    dpi4_value: i32,           // 第4档DPI值
    dpi5_value: i32,           // 第5档DPI值
    dpi6_value: i32,           // 第6档DPI值
    dpi7_value: i32,           // 第7档DPI值
    dpi1_rgb_value: i32,       // 第1档DPIRGB值
    dpi2_rgb_value: i32,       // 第2档DPIRGB值
    dpi3_rgb_value: i32,       // 第3档DPIRGB值
    dpi4_rgb_value: i32,       // 第4档DPIRGB值
    dpi5_rgb_value: i32,       // 第5档DPIRGB值
    dpi6_rgb_value: i32,       // 第6档DPIRGB值
    dpi7_rgb_value: i32,       // 第7档DPIRGB值
    report_rate: u8,           // 回报率值，参考 REPORT_RATE
    silence_height: u8,        // 静默高度(1.0和2.0)
    key_debounce_time: u8,     // 按键响应时间(0 ~ 20)
    sroll_flag: u8,            // 滚轮方向(0: 正向, 1: 反向)
    sleep_time: i32,           // 休眠时间
    high_speed: u8,            // 高速模式(0: 关闭, 1: 打开)
    motion_sync_enable: u8,    // 1:打开Motion Sync 2: 关闭Motion Sync
    angle_snapping_enable: u8, // 1:打开直线修正     0：关闭直线修正
    ripple_control_enable: u8, // 1:打开波纹控制     0：关闭波纹控制（默认）
    move_off_led_enable: u8,   // 1:打开移动关灯功能  0：关闭移动关灯功能
}

type YJXSDK_GetMouseInfo_Type = unsafe extern "C" fn(c_int, *mut YJX_MOUSEINFO);

#[repr(C)]
#[derive(Debug)]
struct YJX_MACROINFO {
    macro_index: i32,         // 宏索引
    record_count: i32,        // 宏记录个数
    data: [YJX_RECORD; 1000], // 宏记录，最大长度1000
}

#[repr(C)]
#[derive(Debug)]
struct YJX_RECORD {
    key_state: i32,
    key_type: i32,
    key_value: i32,
    delay_time: i32,
    move_x: i8,
    move_y: i8, //C中的char如果表示的是单字节字符或者小范围整数值，在Rust中可以映射为i8或u8。在这里，由于moveX和moveY可能有负值（ -15到15），所以使用i8
}

#[repr(C)]
#[derive(Debug)]
struct YJX_KEYINFO {
    profile: i32,
    key_value: i32,
    key_type: i32,
    key_code1: i32,
    key_code2: i32,
    key_code3: i32,
}

type YJXSDK_SetMouseMacro_Type = unsafe extern "C" fn(
    c_int,
    *const YJX_MACROINFO,
    c_int,
) -> c_int;

fn main() {
    // 加载dll
    let library: Library = unsafe { Library::new("YJXMouseSDK.dll") }.expect("Failed to load dll");

    // 获取YJXSDK_Init函数
    let yjx_sdk_init: Symbol<YJXSDK_Init_Type> = unsafe {
        library
            .get(b"YJXSDK_Init\0")
            .expect("Failed to get YJXSDK_Init function")
    };
    unsafe { yjx_sdk_init() };

    // 获取YJXSDK_OpenDevice函数
    let yjx_sdk_open_device: Symbol<YJXSDK_OpenDevice_Type> = unsafe {
        library
            .get(b"YJXSDK_OpenDevice\0")
            .expect("Failed to get YJXSDK_OpenDevice function")
    };

    let vid = 0x25aa;
    let pid = 0x200f;
    let empty_config: OsString = OsString::from("");
    let config_wide: Vec<u16> = empty_config
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // 调用YJXSDK_OpenDevice函数
    let dev_id = unsafe { yjx_sdk_open_device(vid, pid, config_wide.as_ptr()) };
    println!("Result of YJXSDK_OpenDevice: {}", dev_id);

    let yjx_sdk_find_device: Symbol<YJXSDK_FindDevice_Type> = unsafe {
        library
            .get(b"YJXSDK_FindDevice\0")
            .expect("Failed to get YJXSDK_FindDevice function")
    };

    let device_find = unsafe { yjx_sdk_find_device(vid, pid, config_wide.as_ptr()) };
    println!("Result of YJXSDK_FindDevice: {}", device_find);
    let yjx_sdk_device_is_online: Symbol<YJXSDK_DeviceIsOnline_Type> = unsafe {
        library
            .get(b"YJXSDK_DeviceIsOnline\0")
            .expect("Failed to get YJXSDK_DeviceIsOnline function")
    };
    let device_online = unsafe { yjx_sdk_device_is_online(dev_id) };

    println!("Result of YJXSDK_DeviceIsOnline: {}", device_online);

    let yjx_sdk_device_version: Symbol<YJXSDK_DeviceVersion_Type> = unsafe {
        library
            .get(b"YJXSDK_GetDeviceVersion\0")
            .expect("Failed to get YJXSDK_GetDeviceVersion function")
    };

    let mut version: c_int = 0;
    let device_version = unsafe { yjx_sdk_device_version(dev_id, &mut version) };

    println!("Result of YJXSDK_GetDeviceVersion: {}", device_version);

    let yjx_sdk_battery_info: Symbol<YJXSDK_GetDeviceBatteryInfo_Type> = unsafe {
        library
            .get(b"YJXSDK_GetDeviceBatteryInfo\0")
            .expect("Failed to get YJXSDK_GetDeviceBatteryInfo function")
    };

    let mut battery: c_int = 0;
    let mut batter_charging: c_int = 0;

    unsafe {
        yjx_sdk_battery_info(dev_id, &mut battery, &mut batter_charging);
    };

    println!(
        "Result of YJXSDK_GetDeviceBatteryInfo: {}, {}",
        battery, batter_charging
    );

    let yjx_sdk_get_mouse_info: Symbol<YJXSDK_GetMouseInfo_Type> = unsafe {
        library
            .get(b"YJXSDK_GetMouseInfo\0")
            .expect("Failed to get YJXSDK_GetMouseInfo function")
    };

    //结构体必须所有字段赋值
    let mut mouse_info: YJX_MOUSEINFO = YJX_MOUSEINFO {
        profile: 0,
        work_mode: 0,
        is_online: 0,
        battery_value: 0,
        charge_flag: 0,
        light_mode: 0,
        dpi_count: 0,
        dpi_index: 0,
        dpi1_value: 0,
        dpi2_value: 0,
        dpi3_value: 0,
        dpi4_value: 0,
        dpi5_value: 0,
        dpi6_value: 0,
        dpi7_value: 0,
        dpi1_rgb_value: 0,
        dpi2_rgb_value: 0,
        dpi3_rgb_value: 0,
        dpi4_rgb_value: 0,
        dpi5_rgb_value: 0,
        dpi6_rgb_value: 0,
        dpi7_rgb_value: 0,
        report_rate: 0,
        silence_height: 0,
        key_debounce_time: 0,
        sroll_flag: 0,
        sleep_time: 0,
        high_speed: 0,
        motion_sync_enable: 0,
        angle_snapping_enable: 0,
        ripple_control_enable: 0,
        move_off_led_enable: 0,
    };

    unsafe {
        yjx_sdk_get_mouse_info(dev_id, &mut mouse_info);
    };

    println!("Result of YJXSDK_GetMouseInfo: {:?}", mouse_info);
}
