use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;
use crate::{
    components::{ActiveTool, ToolComponent, StrokeType},
    resources::RenovatedToolResources,
    events::ToolChangeEvent,
};


pub fn renovated_tool_management_system(
    mut commands: Commands,
    mut tool_resources: ResMut<RenovatedToolResources>,
    mut tool_events: EventReader<ToolChangeEvent>,
    _scene_tree: SceneTreeRef,
    mut tool_nodes: Query<(Entity, &mut ErasedGd), With<ToolComponent>>,
) {
    for event in tool_events.read() {
        println!("🦀 RUST: renovated_tool_management_system - Processing tool change: {:?}", event);
        
        if let Some(current_tool_entity) = tool_resources.active_tool_entity {
            if let Ok((_, mut erased_gd)) = tool_nodes.get_mut(current_tool_entity) {
                if let Some(mut tool_node) = erased_gd.try_get::<godot::classes::Node>() {
                    tool_node.call("set_enabled", &[false.to_variant()]);
                    println!("🦀 RUST: Deactivated tool entity {:?}", current_tool_entity);
                }
            }
        }
        
        let tool_scene_key = match event.tool_type {
            StrokeType::Brush => "brush",
            StrokeType::Rectangle => "rectangle", 
            StrokeType::Circle => "circle",
            StrokeType::Line => "line",
            StrokeType::Eraser => "eraser",
        };
        
        if let Some(tool_scene) = tool_resources.tool_scenes.get(tool_scene_key) {
            let new_tool_entity = commands.spawn((
                GodotScene::from_resource(tool_scene.clone()),
                ToolComponent {
                    tool_type: match event.tool_type {
                        StrokeType::Brush => 0,
                        StrokeType::Rectangle => 1,
                        StrokeType::Circle => 2,
                        StrokeType::Line => 3,
                        StrokeType::Eraser => 4,
                    },
                    size: event.size,
                    color: event.color,
                },
                ActiveTool,
            )).id();
            
            tool_resources.active_tool_entity = Some(new_tool_entity);
            println!("🦀 RUST: Spawned new tool entity {:?} of type {}", new_tool_entity, tool_scene_key);
        }
    }
    
    for (entity, mut erased_gd) in tool_nodes.iter_mut() {
        if let Some(tool_entity) = tool_resources.active_tool_entity {
            if entity == tool_entity {
                if let Some(mut tool_node) = erased_gd.try_get::<godot::classes::Node>() {
                    tool_node.call("set_enabled", &[true.to_variant()]);
                    println!("🦀 RUST: Updated active tool entity {:?} via ErasedGd", entity);
                }
            }
        }
    }
}
