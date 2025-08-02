use bevy::prelude::*;
use crate::{
    events::CanvasEvent,
    resources::HistoryManager,
    components::Stroke,
};

pub fn history_system(
    mut history: ResMut<HistoryManager>,
    mut canvas_events: EventReader<CanvasEvent>,
    mut commands: Commands,
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
            CanvasEvent::StrokeRemoved { entity } => {
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
            CanvasEvent::StrokeAdded { entity, stroke_data } => {
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
