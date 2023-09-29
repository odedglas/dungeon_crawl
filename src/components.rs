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
