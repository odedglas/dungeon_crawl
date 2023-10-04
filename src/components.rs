pub use crate::prelude::*;
use std::collections::HashSet;

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
pub struct MoveTowardsPlayer;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AmuletOfYala;

#[derive(Debug, PartialEq, Eq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn mark_dirty(&self) -> Self {
        Self::new(self.radius)
    }

    pub fn set_fields_of_view(&mut self, new_fov: HashSet<Point>) {
        self.visible_tiles = new_fov;
        self.is_dirty = false;
    }
}
