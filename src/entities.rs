pub enum GameEntity {
    Player,
    Orc,
    Goblin,
    Oger,
    TwoHeaded,
    AmuletOfYala,
    HealingPotion(i32),
    MapRevealer,
}

impl GameEntity {
    pub fn glyph(&self) -> char {
        match self {
            GameEntity::Player => '@',
            GameEntity::Orc => 'o',
            GameEntity::Goblin => 'g',
            GameEntity::Oger => 'O',
            GameEntity::TwoHeaded => 'E',
            GameEntity::AmuletOfYala => '|',
            GameEntity::HealingPotion(_) => '!',
            GameEntity::MapRevealer => '{',
        }
    }

    pub fn name(&self) -> String {
        match self {
            GameEntity::Player => "Player".to_string(),
            GameEntity::Orc => "Orc".to_string(),
            GameEntity::Goblin => "Goblin".to_string(),
            GameEntity::Oger => "Oger".to_string(),
            GameEntity::TwoHeaded => "Two Headed".to_string(),
            GameEntity::AmuletOfYala => "Amulet of Yala".to_string(),
            GameEntity::HealingPotion(heal_amount) => format!("Healing Potion ({heal_amount}HP)"),
            GameEntity::MapRevealer => "Dungeon Map".to_string(),
        }
    }

    pub fn display(&self) -> (String, char) {
        (self.name(), self.glyph())
    }
}
