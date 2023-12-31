#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub victory_count: u32,
    pub is_human: bool,
}

impl Player {
    pub fn human() -> Self {
        Player {
            name: format!("You"),
            victory_count: 0,
            is_human: true,
        }
    }
    pub fn computer(name: String) -> Self {
        Player {
            name,
            is_human: false,
            victory_count: 0,
        }
    }
}
