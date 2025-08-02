use bevy::prelude::*;
use crate::{
    components::{Stroke, BoundingBox, QuadTreeNode},
    resources::SpatialIndex,
};

pub fn spatial_index_system(
    mut commands: Commands,
    mut spatial_index: ResMut<SpatialIndex>,
    strokes_query: Query<(Entity, &Stroke, &BoundingBox), Changed<BoundingBox>>,
    mut nodes_query: Query<&mut QuadTreeNode>,
) {
    for (entity, _stroke, bounds) in strokes_query.iter() {
        println!("🦀 RUST: spatial_index_system - Processing stroke {:?} with bounds {:?}", entity, bounds);
        
        if spatial_index.root.is_none() {
            let root_bounds = BoundingBox {
                min: Vec2::new(-10000.0, -10000.0),
                max: Vec2::new(10000.0, 10000.0),
            };
            let root_entity = commands.spawn(QuadTreeNode::new(root_bounds, 0)).id();
            spatial_index.root = Some(root_entity);
            println!("🦀 RUST: spatial_index_system - Created root quadtree node {:?}", root_entity);
        }

        if let Some(root) = spatial_index.root {
            println!("🦀 RUST: spatial_index_system - Inserting stroke {:?} into quadtree", entity);
            insert_into_quadtree(
                &mut commands,
                &mut spatial_index,
                &mut nodes_query,
                root,
                entity,
                bounds,
            );
            println!("🦀 RUST: spatial_index_system - Stroke {:?} indexed successfully", entity);
        }
    }
}

fn insert_into_quadtree(
    commands: &mut Commands,
    spatial_index: &mut SpatialIndex,
    nodes_query: &mut Query<&mut QuadTreeNode>,
    node_entity: Entity,
    stroke_entity: Entity,
    bounds: &BoundingBox,
) {
    if let Ok(mut node) = nodes_query.get_mut(node_entity) {
        if !node.intersects(bounds) {
            return;
        }

        if node.children.is_none() && node.entities.len() < spatial_index.max_entities_per_node {
            node.entities.push(stroke_entity);
            spatial_index.insert(stroke_entity, node_entity);
            return;
        }

        if node.children.is_none() && node.depth < spatial_index.max_depth {
            subdivide_node(commands, node_entity, &mut node);
        }

        if let Some(children) = node.children {
            for child in children {
                insert_into_quadtree(commands, spatial_index, nodes_query, child, stroke_entity, bounds);
            }
        }
    }
}

fn subdivide_node(commands: &mut Commands, _parent_entity: Entity, parent_node: &mut QuadTreeNode) {
    let bounds = parent_node.bounds;
    let center = (bounds.min + bounds.max) * 0.5;
    let depth = parent_node.depth + 1;

    let child_bounds = [
        BoundingBox { min: bounds.min, max: center },
        BoundingBox { min: Vec2::new(center.x, bounds.min.y), max: Vec2::new(bounds.max.x, center.y) },
        BoundingBox { min: Vec2::new(bounds.min.x, center.y), max: Vec2::new(center.x, bounds.max.y) },
        BoundingBox { min: center, max: bounds.max },
    ];

    let children = [
        commands.spawn(QuadTreeNode::new(child_bounds[0], depth)).id(),
        commands.spawn(QuadTreeNode::new(child_bounds[1], depth)).id(),
        commands.spawn(QuadTreeNode::new(child_bounds[2], depth)).id(),
        commands.spawn(QuadTreeNode::new(child_bounds[3], depth)).id(),
    ];

    parent_node.children = Some(children);
}
