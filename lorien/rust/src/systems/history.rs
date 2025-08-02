use bevy::prelude::*;
use bevy_godot4::prelude::*;
use crate::{
    events::CanvasEvent,
    resources::HistoryManager,
    components::Stroke,
};

pub fn history_system(
    mut history: ResMut<HistoryManager>,
    mut canvas_events: EventReader<CanvasEvent>,
    _commands: Commands,
) {
    for event in canvas_events.read() {
        println!("🦀 RUST: history_system - Recording canvas event: {:?}", event);
        history.push_event(event.clone());
        println!("🦀 RUST: history_system - History stack size: {}", history.events.len());
    }
}

pub fn undo_system(
    mut history: ResMut<HistoryManager>,
    mut commands: Commands,
    strokes_query: Query<Entity, With<Stroke>>,
) {
    if let Some(event) = history.undo() {
        match event {
            CanvasEvent::StrokeAdded { entity, .. } => {
                if let Ok(entity) = strokes_query.get(*entity) {
                    commands.entity(entity).despawn();
                }
            }
            CanvasEvent::StrokeRemoved { entity: _ } => {
            }
            _ => {}
        }
    }
}

pub fn redo_system(
    mut history: ResMut<HistoryManager>,
    mut commands: Commands,
    strokes_query: Query<Entity, With<Stroke>>,
) {
    if let Some(event) = history.redo() {
        match event {
            CanvasEvent::StrokeAdded { entity: _, stroke_data: _ } => {
            }
            CanvasEvent::StrokeRemoved { entity, .. } => {
                if let Ok(entity) = strokes_query.get(*entity) {
                    commands.entity(entity).despawn();
                }
            }
            _ => {}
        }
    }
}

pub fn renovated_history_system(
    mut history: ResMut<HistoryManager>,
    mut canvas_events: EventReader<CanvasEvent>,
    _commands: Commands,
    _scene_tree: SceneTreeRef,
    _stroke_nodes: Query<(Entity, &mut ErasedGd), With<Stroke>>,
) {
    for event in canvas_events.read() {
        println!("🦀 RUST: renovated_history_system - Recording canvas event: {:?}", event);
        history.push_event(event.clone());
        println!("🦀 RUST: renovated_history_system - History stack size: {}", history.events.len());
    }
}

pub fn renovated_undo_system(
    mut history: ResMut<HistoryManager>,
    mut commands: Commands,
    mut _scene_tree: SceneTreeRef,
    mut stroke_nodes: Query<(Entity, &mut ErasedGd), With<Stroke>>,
) {
    if let Some(event) = history.undo() {
        match event {
            CanvasEvent::StrokeAdded { entity, .. } => {
                if let Ok((stroke_entity, mut erased_gd)) = stroke_nodes.get_mut(*entity) {
                    if let Some(stroke_node) = erased_gd.try_get::<godot::classes::Node2D>() {
                        let _scene_tree_ref = _scene_tree.get();
                        if let Some(mut parent) = stroke_node.get_parent() {
                            parent.remove_child(&stroke_node);
                        }
                        
                        commands.entity(stroke_entity).despawn();
                        println!("🦀 RUST: Undid stroke addition for entity {:?}", stroke_entity);
                    }
                }
            }
            CanvasEvent::StrokeRemoved { entity } => {
                println!("🦀 RUST: Redoing stroke removal for entity {:?}", entity);
            }
            _ => {}
        }
    }
}

pub fn renovated_redo_system(
    mut history: ResMut<HistoryManager>,
    mut commands: Commands,
    _scene_tree: SceneTreeRef,
    stroke_resources: Res<crate::resources::StrokeResources>,
) {
    if let Some(event) = history.redo() {
        match event {
            CanvasEvent::StrokeAdded { entity: _, stroke_data } => {
                let new_stroke_entity = commands.spawn((
                    GodotScene::from_resource(stroke_resources.brush_stroke_scene.clone()),
                    Stroke {
                        points: stroke_data.points.clone(),
                        pressures: stroke_data.pressures.clone(),
                        color: stroke_data.color,
                        size: stroke_data.size,
                        stroke_type: stroke_data.stroke_type,
                        id: fastrand::u64(..),
                    },
                )).id();
                
                println!("🦀 RUST: Redid stroke addition with new entity {:?}", new_stroke_entity);
            }
            CanvasEvent::StrokeRemoved { entity, .. } => {
                println!("🦀 RUST: Redoing stroke removal for entity {:?}", entity);
            }
            _ => {}
        }
    }
}
