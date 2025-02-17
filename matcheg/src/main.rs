#[allow(dead_code)]
enum UserType {
    Admin,
    User,
    Guest,
}
fn main() {
    
    // let user_type = UserType::Admin;
    // match user_type {
    //     UserType::Admin => println!("admin"),
    //     UserType::User => println!("user"),
    //     UserType::Guest => println!("guest"),
    // }
    let a = Some(5);
    println!("pre:{:?}", a);
    if let Some(a) = a {
        println!("current:{}", a);
    } 

    println!("post:{:?}", a);
}
