#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameEntity {
    Player,
    Orc,
    Goblin,
    Oger,
    TwoHeaded,
    AmuletOfYala,
    HealingPotion(i32),
    MapRevealer,
    RustySword,
    ShinySword,
    HugeSword,
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
            GameEntity::RustySword => 's',
            GameEntity::ShinySword => 'S',
            GameEntity::HugeSword => '/',
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
            GameEntity::RustySword => format!("Rusty Sword ({} DMG)", self.get_damage()),
            GameEntity::ShinySword => format!("Shiny Sword ({} DMG)", self.get_damage()),
            GameEntity::HugeSword => format!("Huge Sword ({} DMG)", self.get_damage()),
        }
    }

    pub fn get_damage(&self) -> i32 {
        match self {
            GameEntity::RustySword => 1,
            GameEntity::ShinySword => 2,
            GameEntity::HugeSword => 3,
            _ => 0,
        }
    }

    pub fn display(&self) -> (String, char) {
        (self.name(), self.glyph())
    }
}
