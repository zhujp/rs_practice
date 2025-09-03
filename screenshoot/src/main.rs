// use fs_extra::dir;
// use std::time::Instant;
use xcap::Monitor;

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}

fn main() {
    // let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    // dir::create_all("target/monitors", true).unwrap();

    println!("显示器数量: {:?}", monitors);
    // for monitor in monitors {
    let image = monitors[0].capture_image().unwrap();

    // 新增代码：获取中心像素
    let (width, height) = (image.width(), image.height());
    let mid_x = width / 2;
    let mid_y = height / 2;

    // 修改后的代码片段
    let pixel = image.get_pixel(mid_x, mid_y);
    println!(
        "显示器中心像素 RGB: ({}, {}, {})",
        pixel[0], // R分量
        pixel[1], // G分量
        pixel[2]  // B分量
    );

    // image
    //     .save(format!(
    //         "target/monitors/monitor-{}.png",
    //         normalized(monitor.name().unwrap())
    //     ))
    //     .unwrap();
    // }

    // println!("运行耗时: {:?}", start.elapsed());
}
