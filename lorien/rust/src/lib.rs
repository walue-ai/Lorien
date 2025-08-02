mod components;
mod systems;
mod resources;
mod events;

use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;

use components::*;
use systems::*;
use resources::*;
use events::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct BevyApp {
    base: Base<Node>,
}

#[godot_api]
impl INode for BevyApp {
    fn init(base: Base<Node>) -> Self {
        println!("🦀 RUST: BevyApp Godot class initializing with bevy_godot4...");
        Self { base }
    }

    fn ready(&mut self) {
        println!("🦀 RUST: BevyApp::ready() called - Godot integration active");
    }
}

#[godot_api]
impl BevyApp {
    #[func]
    pub fn spawn_stroke_scene(&self, stroke_data: Dictionary) {
        println!("🦀 RUST: spawn_stroke_scene called with data: {:?}", stroke_data);
    }

    #[func]
    pub fn update_tool_state(&self, tool_data: Dictionary) {
        println!("🦀 RUST: update_tool_state called with data: {:?}", tool_data);
    }

    #[func]
    pub fn manage_scene_tree(&self, operation: String, data: Dictionary) {
        println!("🦀 RUST: manage_scene_tree called: {} with data: {:?}", operation, data);
    }

    #[func]
    pub fn get_strokes_in_region(&self, region_data: Dictionary) -> Array<Dictionary> {
        println!("🦀 RUST: get_strokes_in_region called with data: {:?}", region_data);
        Array::new()
    }

    #[func]
    pub fn set_canvas_transform(&self, transform_data: Dictionary) {
        println!("🦀 RUST: set_canvas_transform called with data: {:?}", transform_data);
        
        let zoom = transform_data.get("zoom").unwrap_or(Variant::nil());
        let offset = transform_data.get("offset").unwrap_or(Variant::nil());
        let viewport_size = transform_data.get("viewport_size").unwrap_or(Variant::nil());
        
        println!("🦀 RUST: Canvas transform - Zoom: {:?}, Offset: {:?}, Size: {:?}", 
                zoom, offset, viewport_size);
    }
}

#[bevy_app]
fn build_app(app: &mut App) {
    println!("🦀 RUST: Initializing renovated Bevy ECS with bevy_godot4 architecture");
    
    app
        .init_resource::<crate::components::ActiveTool>()
        .init_resource::<CanvasState>()
        .init_resource::<StrokeResources>()
        .init_resource::<RenovatedToolResources>()
        .init_resource::<SpatialIndex>()
        .init_resource::<HistoryManager>()
        .add_event::<CanvasEvent>()
        .add_event::<ToolChangeEvent>()
        .add_event::<StrokeInputEvent>()
        .add_systems(Update, (
            renovated_stroke_spawning_system,
            renovated_tool_management_system,
            renovated_scene_tree_system,
        ))
        .add_systems(Update, spatial_index_system)
        .add_systems(Update, renovated_history_system);
    
    println!("🦀 RUST: Renovated bevy_godot4 systems registered successfully");
    println!("🦀 RUST: - Renovated stroke spawning system: ACTIVE");
    println!("🦀 RUST: - Renovated tool management system: ACTIVE");
    println!("🦀 RUST: - Renovated scene tree system: ACTIVE");
    println!("🦀 RUST: - Spatial index system: ACTIVE");
    println!("🦀 RUST: - Renovated history system: ACTIVE");
}
