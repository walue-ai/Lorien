use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;
use crate::{
    components::{Stroke, StrokeVisual, Dirty, StrokeType},
    resources::StrokeResources,
    events::StrokeInputEvent,
};


pub fn renovated_stroke_spawning_system(
    mut commands: Commands,
    stroke_resources: Res<StrokeResources>,
    mut stroke_events: EventReader<StrokeInputEvent>,
    mut _scene_tree: SceneTreeRef,
) {
    for event in stroke_events.read() {
        println!("🦀 RUST: renovated_stroke_spawning_system - Processing stroke event: {:?}", event);
        
        if event.is_start {
            let stroke_entity = commands.spawn((
                GodotScene::from_resource(stroke_resources.brush_stroke_scene.clone()),
                Stroke::new(
                    StrokeType::Brush,
                    godot::builtin::Color::BLACK,
                    5.0
                ),
                Dirty,
            )).id();
            
            println!("🦀 RUST: Spawned new stroke entity {:?} using GodotScene", stroke_entity);
        }
    }
}

pub fn renovated_scene_tree_system(
    mut commands: Commands,
    mut _scene_tree: SceneTreeRef,
    mut stroke_scenes: Query<(Entity, &mut ErasedGd), (With<Stroke>, Without<StrokeVisual>)>,
) {
    for (entity, mut erased_gd) in stroke_scenes.iter_mut() {
        if let Some(stroke_node) = erased_gd.try_get::<godot::classes::Node2D>() {
            println!("🦀 RUST: Managing stroke node {:?} via SceneTreeRef", entity);
            
            let scene_tree_ref = _scene_tree.get();
            if let Some(_strokes_parent) = scene_tree_ref
                .get_root()
                .unwrap()
                .get_node_or_null("InfiniteCanvas/SubViewport/Strokes") 
            {
                println!("🦀 RUST: Stroke node managed by bevy_godot4 scene system");
            }
            
            commands.entity(entity).insert(StrokeVisual { 
                godot_node: ErasedGd::new(stroke_node.upcast::<godot::classes::Node>()) 
            });
        }
    }
}
