use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct StrokeInputEvent {
    pub position: Vec2,
    pub pressure: f32,
    pub is_start: bool,
    pub is_end: bool,
}
