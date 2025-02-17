fn main() {
    // let tup: (i32, f64, u8) = (20, 0.63, 1); //元组可以存储不同的类型
    // println!("{:?}", tup);
    let s = String::from("hello vilay");
    let (str, len) = do_str(s);
    println!("{} {}", str, len);
}

fn do_str(str: String) -> (String, usize) {
    let s = str.len();
    (str, s)
}
