use bevy::prelude::*;
use godot::builtin::Color;
use crate::components::StrokeType;

#[derive(Resource)]
pub struct ActiveTool {
    pub tool_type: StrokeType,
    pub brush_size: f32,
    pub brush_color: Color,
    pub opacity: f32,
}

impl Default for ActiveTool {
    fn default() -> Self {
        Self {
            tool_type: StrokeType::Brush,
            brush_size: 5.0,
            brush_color: Color::BLACK,
            opacity: 1.0,
        }
    }
}
