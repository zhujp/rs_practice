use clipboard_rs::{common::RustImage, Clipboard, ClipboardContext, ContentFormat};
// use clipboard_rs::{common::RustImage, Clipboard, ClipboardContext};

#[cfg(target_os = "windows")]
const TMP_PATH: &str = "C:\\Windows\\Temp\\";

#[cfg(target_os = "macos")]
const TMP_PATH: &str = "/tmp/";
fn main() {
    let ctx = ClipboardContext::new().unwrap();
    let types = ctx.available_formats().unwrap();
    println!("{:?}", types);

    let content = ctx.get_text().unwrap_or("".to_string());
    println!("txt={}", content);

    let has_file = ctx.has(ContentFormat::Files);
    println!("has_file={}", has_file);

    let files = ctx.get_files().unwrap_or_default();
    println!("files={:?}", files);

    // let mut dest_files: Vec<String> = Vec::new();
    // dest_files.push("C:\\Users\\xgb1\\Desktop\\web\\office\\test.png".to_string());

    // ctx.set_files(dest_files).unwrap(); //success

    // let has_img = ctx.has(ContentFormat::Image);
    // println!("has_img={}", has_img);
    // let img = ctx.get_image();
    // match img {
    //     Ok(img) => {
    //         let _ = img
    //             .save_to_path(format!("{}clipboard.png", TMP_PATH).as_str())
    //             .map_err(|e| println!("err={:?}", e));
    //     }
    //     Err(err) => {
    //         println!("err={:?}", err);
    //     }
    // }
}

//文件：复制文件/文件夹  压缩文件files只有一个元素，如果是文件夹，就是目录元素，如果是多个文件则是多个元素
//图片：截图
//文本:复制文本/url链接
//html：富文本
