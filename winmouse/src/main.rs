use std::mem::size_of;
use std::mem::MaybeUninit;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::um::winuser::{
    SystemParametersInfoW, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE, SPI_GETMOUSESPEED,
    SPI_GETWHEELSCROLLLINES, SPI_SETMOUSESPEED, SPI_SETWHEELSCROLLLINES,
};
fn main() {
    println!("Hello, world!");
    // set_mouse_speed(10);
    // set_mouse_wheel(5); //设置为滚动多少行
    set_mouse_wheel(0xFFFFFFFF); //设置为滚动整个屏幕
}

fn set_mouse_speed(speed: u32) {
    let mut current_speed: u32 = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe {
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            &mut current_speed as *mut u32 as *mut c_void,
            0,
        );
    }

    println!("current speed:{}", current_speed);

    let result =
        unsafe { SystemParametersInfoW(SPI_SETMOUSESPEED, 0, speed as *mut _, SPIF_UPDATEINIFILE) };

    if result == 0 {
        println!("set mouse speed failed");
    } else {
        println!("set mouse speed success");
    }
}

fn set_mouse_wheel(lines: u32) {
    let mut current_scroll_lines: u32 = 0;
    unsafe {
        SystemParametersInfoW(
            SPI_GETWHEELSCROLLLINES,
            0,
            &mut current_scroll_lines as *mut _ as _,
            0,
        );
    }
    println!("Current wheel scroll lines: {}", current_scroll_lines); //success

    let result = unsafe {
        SystemParametersInfoW(
            SPI_SETWHEELSCROLLLINES,
            lines,
            // lines as *mut _,
            std::ptr::null_mut(),
            SPIF_UPDATEINIFILE,
        )
    }; //success

    if result == 0 {
        println!("Failed to set wheel scroll lines");
    } else {
        println!("Successfully set wheel scroll lines to {}", lines);
    }
}
