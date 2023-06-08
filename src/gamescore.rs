#[derive(Clone)]
pub struct GameScore {
    pub score: u32,
    pub level: u32,
}

impl GameScore {
    pub fn new() -> Self {
        GameScore { score: 0, level: 0 }
    }
}
