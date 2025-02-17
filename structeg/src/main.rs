#[derive(Debug)]
struct Reactangel {
    width: u32,
    length: u32,
}

impl Reactangel {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn square(size: u32) -> Reactangel {
        Reactangel {
            width: size,
            length: size,
        }
    }
}
fn main() {
    let rect = Reactangel {
        width: 30,
        length: 50,
    };

    let rect2 = Reactangel::square(10);

    print!("{}", rect.area());

    print!("{}", rect2.area());

    print!("{:#?}", rect);
}

