use bevy::prelude::*;
use super::stroke::BoundingBox;

#[derive(Component)]
pub struct QuadTreeNode {
    pub bounds: BoundingBox,
    pub entities: Vec<Entity>,
    pub children: Option<[Entity; 4]>,
    pub depth: u32,
}

impl QuadTreeNode {
    pub fn new(bounds: BoundingBox, depth: u32) -> Self {
        Self {
            bounds,
            entities: Vec::new(),
            children: None,
            depth,
        }
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        point.x >= self.bounds.min.x
            && point.x <= self.bounds.max.x
            && point.y >= self.bounds.min.y
            && point.y <= self.bounds.max.y
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        !(other.max.x < self.bounds.min.x
            || other.min.x > self.bounds.max.x
            || other.max.y < self.bounds.min.y
            || other.min.y > self.bounds.max.y)
    }
}
