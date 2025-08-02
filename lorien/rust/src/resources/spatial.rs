use bevy::prelude::*;
use crate::components::BoundingBox;
use std::collections::HashMap;

#[derive(Resource)]
pub struct SpatialIndex {
    pub root: Option<Entity>,
    pub entity_to_node: HashMap<Entity, Entity>,
    pub max_depth: u32,
    pub max_entities_per_node: usize,
}

impl Default for SpatialIndex {
    fn default() -> Self {
        Self {
            root: None,
            entity_to_node: HashMap::new(),
            max_depth: 8,
            max_entities_per_node: 10,
        }
    }
}

impl SpatialIndex {
    pub fn new(_bounds: BoundingBox) -> Self {
        Self {
            root: None,
            entity_to_node: HashMap::new(),
            max_depth: 8,
            max_entities_per_node: 10,
        }
    }

    pub fn insert(&mut self, entity: Entity, node_entity: Entity) {
        self.entity_to_node.insert(entity, node_entity);
    }

    pub fn remove(&mut self, entity: Entity) {
        self.entity_to_node.remove(&entity);
    }

    pub fn query_region(&self, _bounds: &BoundingBox) -> Vec<Entity> {
        Vec::new()
    }
}
