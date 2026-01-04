/*
 fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest { largest=item; }
    }
    largest
}
*/
#[derive(Debug)]
struct Pointy<T> {
    x: T,
    y: T,
}

impl<T> Pointy<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let int_pointy = Pointy { x: 5, y:10 };
    println!("{int_pointy:?}, {}", int_pointy.x());

}
