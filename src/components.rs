pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player; // Tag

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Monster; // Tag

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RandomMovement;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Movement {
    Random,
    Keyboard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveIntent {
    pub entity: Entity,
    pub to: Point,
    pub movement_type: Movement,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttackIntent {
    pub target: Entity,
    pub attacker: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct EntityName(pub String);
