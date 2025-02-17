fn main() {
    let mut s = String::from("hello");
    do_str(&mut s);
    println!("{}", s); //通过借用规则，可变引用，可以改变s的值


fn do_str(str: &mut String) {
    println!("{}", str);
    str.push_str(" vilay");
}