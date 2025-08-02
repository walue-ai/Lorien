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
        println!("🦀 RUST: BevyApp Godot class initializing...");
        Self { base }
    }

    fn ready(&mut self) {
        println!("🦀 RUST: BevyApp::ready() called - Godot integration active");
    }
}

#[godot_api]
impl BevyApp {
    #[func]
    pub fn send_stroke_input_event(&mut self, event_data: Dictionary) {
        println!("🦀 RUST: send_stroke_input_event called with data: {:?}", event_data);
        
        let position = event_data.get("position").unwrap_or(Variant::nil());
        let pressure = event_data.get("pressure").unwrap_or(Variant::nil());
        let is_start = event_data.get("is_start").unwrap_or(Variant::nil());
        let is_end = event_data.get("is_end").unwrap_or(Variant::nil());
        
        println!("🦀 RUST: Stroke input - Position: {:?}, Pressure: {:?}, Start: {:?}, End: {:?}", 
                position, pressure, is_start, is_end);
    }

    #[func]
    pub fn send_tool_change_event(&mut self, tool_data: Dictionary) {
        println!("🦀 RUST: send_tool_change_event called with data: {:?}", tool_data);
        
        let tool_type = tool_data.get("tool_type").unwrap_or(Variant::nil());
        let size = tool_data.get("size").unwrap_or(Variant::nil());
        let color = tool_data.get("color").unwrap_or(Variant::nil());
        
        println!("🦀 RUST: Tool change - Type: {:?}, Size: {:?}, Color: {:?}", 
                tool_type, size, color);
    }

    #[func]
    pub fn send_undo_event(&mut self) {
        println!("🦀 RUST: send_undo_event called");
    }

    #[func]
    pub fn send_redo_event(&mut self) {
        println!("🦀 RUST: send_redo_event called");
    }

    #[func]
    pub fn send_clear_event(&mut self) {
        println!("🦀 RUST: send_clear_event called");
    }

    #[func]
    pub fn get_strokes_in_region(&mut self, region_data: Dictionary) -> Array<Dictionary> {
        println!("🦀 RUST: get_strokes_in_region called with data: {:?}", region_data);
        Array::new()
    }

    #[func]
    pub fn set_canvas_transform(&mut self, transform_data: Dictionary) {
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
    println!("🦀 RUST: Initializing Bevy ECS for Lorien infinite canvas");
    
    app.add_plugins(MinimalPlugins)
        .init_resource::<ActiveTool>()
        .init_resource::<CanvasState>()
        .init_resource::<SpatialIndex>()
        .init_resource::<HistoryManager>()
        .add_event::<CanvasEvent>()
        .add_event::<ToolChangeEvent>()
        .add_event::<StrokeInputEvent>()
        .add_systems(Update, (
            tool_input_system,
            stroke_creation_system,
            stroke_optimization_system,
        ))
        .add_systems(Update, spatial_index_system)
        .add_systems(Update, history_system)
        .add_systems(Update, stroke_rendering_system);
    
    println!("🦀 RUST: Bevy ECS systems registered successfully");
    println!("🦀 RUST: - Tool input system: ACTIVE");
    println!("🦀 RUST: - Stroke creation system: ACTIVE");
    println!("🦀 RUST: - Stroke optimization system: ACTIVE");
    println!("🦀 RUST: - Spatial index system: ACTIVE");
    println!("🦀 RUST: - History management system: ACTIVE");
    println!("🦀 RUST: - Stroke rendering system: ACTIVE");
}
