// #[allow(dead_code)]
// #[derive(Debug)]
// //隐式值，从0开始
// enum OrderStatus {
//     Pending,
//     Processing,
//     Shipped,
//     Delivered,
//     Cancelled,
// }

// #[allow(dead_code)]
// #[derive(Debug)]
// //显示指定值
// enum PayStatus {
//     Pending = 0,
//     Paid = 10,
//     Refunded = 20,
// }
// fn main() {
//     println!("pending:{:?}", OrderStatus::Pending as u32); //0
//                                                            // println!("pending:{:?}", OrderStatus::Shipped as u32);  //2
//     println!("Paid:{:?}", PayStatus::Paid as u32); //10

//     // use OrderStatus::*;  //全部导入，之间使用变量
//     use PayStatus::{Paid, Pending}; //部分导入
//     println!("{:?}", Pending as u32); //输出Pending,加as u32才会变成数字
// }

fn main() {
    let a = Some(1);
    let b = add(a);
    println!("{:?}", b);

}

fn add(a:Option<i32>)->Option<i32>{
    match a{
        Some(x)=>Some(x+1),
        None=>None,
    }
}