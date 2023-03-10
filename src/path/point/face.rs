pub enum Face {
    UP,
    LEFT,
    RIGHT,
    DOWN
}

impl Face {
    pub fn get_all() -> Vec<Face> {
        return vec![
            Face::DOWN,
            Face::LEFT,
            Face::RIGHT,
            Face::DOWN
        ];
    }
}