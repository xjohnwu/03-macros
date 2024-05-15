use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}

fn main() {
    let up: Direction = DirectionUp::new(10).into();
    let left: Direction = 12.into();
    println!("up: {:?}, left: {:?}", up, left);
}
