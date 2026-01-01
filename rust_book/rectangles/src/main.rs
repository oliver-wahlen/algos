#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.w * self.h
    }
}

fn main() {
    let rect1 = Rect {
        w: 30,
        h: 50,
    };
    println!("Area {}", area(&rect1));
    println!("Area method {}", rect1.area());
    println!("Rect {:#?}", rect1);
}

fn area (rect: &Rect) -> u32 {
    rect.w*rect.h
}
