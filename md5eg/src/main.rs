use md5::{Md5, Digest};

/// 计算给定字符串的 MD5 哈希值并返回其十六进制表示形式。
///
/// # 参数
/// * `input` - 需要计算 MD5 哈希的字符串。
///
/// # 返回值
/// 返回输入字符串的 MD5 哈希值的十六进制字符串表示形式。
pub fn generate_md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    let input = "hello world";
    println!("The MD5 hash of '{}' is: {}", input, generate_md5(input));
}