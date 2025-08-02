use bevy::prelude::*;
use godot::builtin::Color;
use bevy_godot4::prelude::*;

#[derive(Component, Clone)]
pub struct Stroke {
    pub points: Vec<Vec2>,
    pub pressures: Vec<f32>,
    pub color: Color,
    pub size: f32,
    pub stroke_type: StrokeType,
    pub id: u64,
}

#[derive(Component)]
pub struct StrokeVisual {
    pub godot_node: ErasedGd,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct Dirty;

#[derive(Component, Clone, Copy, Debug)]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(Debug)]
pub enum StrokeType {
    Brush,
    Eraser,
    Rectangle,
    Circle,
    Line,
}

impl Stroke {
    pub fn new(stroke_type: StrokeType, color: Color, size: f32) -> Self {
        Self {
            points: Vec::new(),
            pressures: Vec::new(),
            color,
            size,
            stroke_type,
            id: fastrand::u64(..),
        }
    }

    pub fn add_point(&mut self, point: Vec2, pressure: f32) {
        self.points.push(point);
        self.pressures.push(pressure);
    }

    pub fn calculate_bounds(&self) -> Option<BoundingBox> {
        if self.points.is_empty() {
            return None;
        }

        let mut min = self.points[0];
        let mut max = self.points[0];

        for point in &self.points {
            min = min.min(*point);
            max = max.max(*point);
        }

        let padding = self.size * 0.5;
        Some(BoundingBox {
            min: min - Vec2::splat(padding),
            max: max + Vec2::splat(padding),
        })
    }
}
