use std::env;

fn main() {
    if cfg!(target_os = "windows") {
        // 在Windows上，通常使用TEMP或TMP环境变量表示临时目录
        if let Ok(temp_dir) = env::var("TEMP") {
            println!("Windows temp directory: {}", temp_dir);
        } else {
            println!("Failed to get Windows temp directory");
        }
    } else if cfg!(target_os = "macos") {
        // 在macOS上，临时文件通常位于 /var/folders 体系下，可以通过 TMPDIR 环境变量获取
        if let Ok(temp_dir) = env::var("TMPDIR") {
            println!("MacOS temp directory: {}", temp_dir);
        } else {
            println!("Failed to get MacOS temp directory");
        }
    } else {
        println!("Unsupported OS");
    }
}
