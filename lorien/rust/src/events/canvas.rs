use bevy::prelude::*;
use godot::builtin::Color;
use crate::components::{Stroke, StrokeType};

#[derive(Event, Clone, Debug)]
pub enum CanvasEvent {
    StrokeAdded {
        entity: Entity,
        stroke_data: StrokeData,
    },
    StrokeRemoved {
        entity: Entity,
    },
    StrokeModified {
        entity: Entity,
        old_data: StrokeData,
        new_data: StrokeData,
    },
    Clear,
}

#[derive(Clone, Debug)]
pub struct StrokeData {
    pub points: Vec<Vec2>,
    pub pressures: Vec<f32>,
    pub color: Color,
    pub size: f32,
    pub stroke_type: StrokeType,
}

impl From<&Stroke> for StrokeData {
    fn from(stroke: &Stroke) -> Self {
        Self {
            points: stroke.points.clone(),
            pressures: stroke.pressures.clone(),
            color: stroke.color,
            size: stroke.size,
            stroke_type: stroke.stroke_type,
        }
    }
}
