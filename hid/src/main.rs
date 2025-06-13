extern crate hidapi;

use hidapi::{HidApi, HidDevice};
use std::error::Error;
use std::ffi::CString;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Printing all available hid devices:");

    // match HidApi::new() {
    //     Ok(api) => {
    //         for device in api.device_list() {
    //             println!(
    //                 "{:04x}:{:04x}:{:?}",
    //                 device.vendor_id(),
    //                 device.product_id(),
    //                 device.path()
    //             );
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //     }
    // }
    let hid_api = HidApi::new()?;

    // 打开设备（替换为实际VID/PID）
    // let device = hid_api.open(0xxx, 0xxx)?;
    //success
    // let path_to_open = "\\\\?\\hid#vid_xx&pid_xx&mi_01&col05#7&1886dc89&0&0004#{4d1e55b2-f16f-11cf-88cb-001111000030}";
    // let c_str_path = CString::new(path_to_open).expect("Failed to create CString");
    // let device = hid_api.open_path(&c_str_path)?;

    //通过path方式打开设备发送数据
    let path_to_open = "\\\\?\\HID#VID_00&PID_01B9&MI_01&Col08#7&2e4aef99&0&0007#{4d1e55b2-f16f-11cf-88cb-001111000030}";
    let c_str_path = CString::new(path_to_open).expect("Failed to create CString");
    let device = hid_api.open_path(&c_str_path)?;

    // let mut buffer = [0u8; 520];
    // match device.get_report_descriptor(&mut buffer) {
    //     Ok(bytes_written) => {
    //         println!("成功获取 {} 字节的报告描述符:", bytes_written);

    //         let descriptor: &[u8] = &buffer[..bytes_written];
    //         // println!("描述符内容: {:?}", descriptor);
    //         for byte in descriptor {
    //             print!("{:02X} ", byte);
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("获取描述符失败: {}", e);
    //         return Err(Box::new(e) as Box<dyn Error>);
    //     }
    // }

    let mut rgb_data = [[0u8; 3]; 126];
    for key in &mut rgb_data {
        *key = [0x2d, 0x9f, 0x56]; // R=255, G=0, B=0
    }

    // 发送灯效数据
    send_music_lighting(&device, &rgb_data)?;

    // send_command(&device);

    Ok(())
}

fn send_music_lighting(
    device: &HidDevice,
    rgb_data: &[[u8; 3]; 126],
) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0u8; 520];

    buffer[0] = 0x09;

    let command_header = [0x08, 0x00, 0x00, 0x01, 0x00, 0x7a, 0x01];
    buffer[1..=7].copy_from_slice(&command_header);

    // 写入RGB数据 (索引8-385)
    let rgb_bytes = rgb_data
        .iter()
        .flat_map(|&[r, g, b]| vec![r, g, b])
        .collect::<Vec<u8>>();
    buffer[8..386].copy_from_slice(&rgb_bytes);

    println!("Sending {:?}", buffer);
    let result = device.send_feature_report(&buffer)?;
    Ok(())
}

fn send_command(device: &HidDevice) {
    let mut data = [
        0x08, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x49,
    ]; // 示例数据

    let result = device.write(&data);

    println!("Sending {:?}", result);
    println!("Sending {:?}", data);
}
