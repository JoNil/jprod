pub struct Scissor {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct Pso {
    pub scissor: Option<Scissor>,
}

impl Pso {
    pub fn new() -> Pso {
        Pso {
            scissor: None,
        }
    }
}