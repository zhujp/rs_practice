use std::{
    fs::{self, canonicalize, File},
    io::{self, Write},
    path::{Component, Path},
};
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

fn main() -> io::Result<()> {
    // 指定要压缩的文件夹路径和输出的ZIP文件名
    let src_folder = "C:\\Users\\xgb1\\Desktop\\罗技商务";
    let output_zip = "./output.zip";

    // zip_dir(src_folder, output_zip);
    dir_extract(output_zip, "./output");
    println!("压缩完成！");
    Ok(())
}

fn zip_dir(src_path: &str, output_path: &str) -> io::Result<()> {
    let src_folder = canonicalize(src_path)?;
    let file = File::create(output_path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Bzip2)
        .unix_permissions(0o755);

    for entry in WalkDir::new(&src_path).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        // 更安全的路径前缀处理
        let name = path
            .strip_prefix(&src_path)
            .unwrap_or_else(|_| panic!("路径前缀不匹配: {:?}", path));

        // 转换路径分隔符为ZIP标准格式
        let mut name_str = name.to_string_lossy().replace("\\", "/");

        // 如果是目录则添加斜杠后缀
        if path.is_dir() && !name_str.ends_with('/') {
            name_str.push('/');
        }

        if path.is_dir() {
            zip.add_directory(name_str, options)?;
        } else {
            zip.start_file(name_str, options)?;
            let mut f = File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}

fn dir_extract(zip_file: &str, extract_to: &str) -> io::Result<()> {
    fs::create_dir_all(extract_to)?;
    let file = File::open(zip_file)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let raw_name = file.name().to_owned();

        // 路径安全检测
        let path = Path::new(&raw_name);
        if path.components().any(|c| matches!(c, Component::ParentDir)) {
            println!("[安全警告] 跳过包含上级目录的路径: {}", raw_name);
            continue;
        }

        // 构建完整解压路径
        let target_path = Path::new(extract_to).join(path);

        // 二次路径安全检查
        if !target_path.starts_with(extract_to) {
            println!("[安全警告] 跳过越界路径: {}", raw_name);
            continue;
        }

        // 处理目录（以/结尾的条目）
        if file.name().ends_with('/') {
            fs::create_dir_all(&target_path)?;
        } else {
            // 确保父目录存在
            if let Some(parent) = target_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            // 写入文件内容
            let mut out_file = File::create(&target_path)?;
            io::copy(&mut file, &mut out_file)?;
        }

        // 保留文件权限（Unix系统）
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&target_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}
