use macros::EnumFromDarling;

#[allow(unused)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(T),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction<u32> = DirectionUp::new(10).into();
    let left: Direction<u32> = 12.into();
    println!("up: {:?}, left: {:?}", up, left);
}
