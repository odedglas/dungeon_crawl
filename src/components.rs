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
    Keyboard(VirtualKeyCode),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MoveIntent {
    pub entity: Entity,
    pub current: Point,
    pub movement_type: Movement,
}
