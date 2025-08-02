use bevy::prelude::*;
use godot::builtin::Color;
use crate::components::StrokeType;

#[derive(Event, Debug)]
pub struct ToolChangeEvent {
    pub tool_type: StrokeType,
    pub size: f32,
    pub color: Color,
    pub opacity: f32,
}
