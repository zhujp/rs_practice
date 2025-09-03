extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        // res.set_icon("path/to/icon.ico"); // 如果有图标的话
        res.add_binary("./HIDUsb.dll", "LIBRARY1.DLL");
        res.add_binary("./ucrtbased.dll", "LIBRARY2.DLL");
        res.compile().unwrap();
    }
}