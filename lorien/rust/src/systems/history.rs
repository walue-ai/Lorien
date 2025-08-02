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
        history.push_event(event.clone());
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
