#[no_mangle]
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

// #[no_mangle]
// pub extern "C" fn get_from_clipboard() -> Vec<String> {
//     let mut contents = Vec::new();

//     contents    
//         .push("Hello".to_string());
    
// }

// #[no_mangle]
// pub extern "C" fn set_to_clipboard(text: &str) {
    
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
