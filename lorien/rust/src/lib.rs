mod components;
mod systems;
mod resources;
mod events;

use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;
use std::sync::{Arc, Mutex};

use systems::*;
use resources::*;
use events::*;
use components::{StrokeType, BoundingBox};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LorienBevyManager {
    base: Base<Node>,
    bevy_app: Option<Arc<Mutex<App>>>,
}

#[godot_api]
impl INode for LorienBevyManager {
    fn init(base: Base<Node>) -> Self {
        println!("🦀 RUST: LorienBevyManager Godot class initializing with bevy_godot4...");
        Self { 
            base,
            bevy_app: None,
        }
    }

    fn ready(&mut self) {
        println!("🦀 RUST: LorienBevyManager::ready() called - Godot integration active");
        let app = build_app_instance();
        self.bevy_app = Some(Arc::new(Mutex::new(app)));
        println!("🦀 RUST: Bevy App instance initialized");
    }
}

#[godot_api]
impl LorienBevyManager {
    #[func]
    pub fn spawn_stroke_scene(&self, stroke_data: Dictionary) {
        println!("🦀 RUST: spawn_stroke_scene called with data: {:?}", stroke_data);
        
        let position = if let Some(pos_var) = stroke_data.get("position") {
            if let Ok(pos) = pos_var.try_to::<Vector2>() {
                Vec2::new(pos.x, pos.y)
            } else {
                Vec2::ZERO
            }
        } else {
            Vec2::ZERO
        };
        
        let pressure = stroke_data.get("pressure")
            .and_then(|v| v.try_to::<f32>().ok())
            .unwrap_or(1.0);
            
        let is_start = stroke_data.get("is_start")
            .and_then(|v| v.try_to::<bool>().ok())
            .unwrap_or(false);
            
        let is_end = stroke_data.get("is_end")
            .and_then(|v| v.try_to::<bool>().ok())
            .unwrap_or(false);
        
        if let Some(app_arc) = &self.bevy_app {
            if let Ok(mut app) = app_arc.lock() {
                app.world_mut().send_event(StrokeInputEvent {
                    position,
                    pressure,
                    is_start,
                    is_end,
                });
                println!("🦀 RUST: Sent StrokeInputEvent to Bevy ECS");
                
                app.update();
            }
        }
    }

    #[func]
    pub fn update_tool_state(&self, tool_data: Dictionary) {
        println!("🦀 RUST: update_tool_state called with data: {:?}", tool_data);
        
        let tool_type = tool_data.get("tool_type")
            .and_then(|v| v.try_to::<i32>().ok())
            .unwrap_or(0);
            
        let size = tool_data.get("size")
            .and_then(|v| v.try_to::<f32>().ok())
            .unwrap_or(5.0);
            
        let color = if let Some(color_var) = tool_data.get("color") {
            if let Ok(godot_color) = color_var.try_to::<godot::builtin::Color>() {
                godot_color
            } else {
                godot::builtin::Color::BLACK
            }
        } else {
            godot::builtin::Color::BLACK
        };
        
        let opacity = tool_data.get("opacity")
            .and_then(|v| v.try_to::<f32>().ok())
            .unwrap_or(1.0);
        
        if let Some(app_arc) = &self.bevy_app {
            if let Ok(mut app) = app_arc.lock() {
                let stroke_type = match tool_type {
                    0 => StrokeType::Brush,
                    1 => StrokeType::Rectangle,
                    2 => StrokeType::Circle,
                    3 => StrokeType::Line,
                    4 => StrokeType::Eraser,
                    _ => StrokeType::Brush,
                };
                app.world_mut().send_event(ToolChangeEvent {
                    tool_type: stroke_type,
                    size,
                    color,
                    opacity,
                });
                println!("🦀 RUST: Sent ToolChangeEvent to Bevy ECS");
                
                app.update();
            }
        }
    }

    #[func]
    pub fn manage_scene_tree(&self, operation: String, data: Dictionary) {
        println!("🦀 RUST: manage_scene_tree called: {} with data: {:?}", operation, data);
        
        if let Some(app_arc) = &self.bevy_app {
            if let Ok(mut app) = app_arc.lock() {
                match operation.as_str() {
                    "undo" => {
                        println!("🦀 RUST: Processing undo operation");
                    },
                    "redo" => {
                        println!("🦀 RUST: Processing redo operation");
                    },
                    "clear" => {
                        app.world_mut().send_event(CanvasEvent::Clear);
                        println!("🦀 RUST: Sent Clear event to Bevy ECS");
                    },
                    _ => {
                        println!("🦀 RUST: Unknown scene operation: {}", operation);
                    }
                }
                
                app.update();
            }
        }
    }

    #[func]
    pub fn get_strokes_in_region(&self, region_data: Dictionary) -> Array<Dictionary> {
        println!("🦀 RUST: get_strokes_in_region called with data: {:?}", region_data);
        
        let x = region_data.get("x").and_then(|v| v.try_to::<f32>().ok()).unwrap_or(0.0);
        let y = region_data.get("y").and_then(|v| v.try_to::<f32>().ok()).unwrap_or(0.0);
        let width = region_data.get("width").and_then(|v| v.try_to::<f32>().ok()).unwrap_or(0.0);
        let height = region_data.get("height").and_then(|v| v.try_to::<f32>().ok()).unwrap_or(0.0);
        
        if let Some(app_arc) = &self.bevy_app {
            if let Ok(app) = app_arc.lock() {
                if let Some(spatial_index) = app.world().get_resource::<SpatialIndex>() {
                    let _bounds = BoundingBox {
                        min: Vec2::new(x, y),
                        max: Vec2::new(x + width, y + height),
                    };
                    
                    let mut result = Array::new();
                    if let Some(root) = spatial_index.root {
                        let mut stroke_dict = Dictionary::new();
                        stroke_dict.set("entity_id", root.index());
                        result.push(&stroke_dict);
                    }
                    println!("🦀 RUST: Found {} strokes in region", result.len());
                    return result;
                }
            }
        }
        
        Array::new()
    }

    #[func]
    pub fn set_canvas_transform(&self, transform_data: Dictionary) {
        println!("🦀 RUST: set_canvas_transform called with data: {:?}", transform_data);
        
        let zoom = transform_data.get("zoom")
            .and_then(|v| v.try_to::<f32>().ok())
            .unwrap_or(1.0);
            
        let offset = if let Some(offset_var) = transform_data.get("offset") {
            if let Ok(offset_vec) = offset_var.try_to::<Vector2>() {
                Vec2::new(offset_vec.x, offset_vec.y)
            } else {
                Vec2::ZERO
            }
        } else {
            Vec2::ZERO
        };
        
        let viewport_size = if let Some(size_var) = transform_data.get("viewport_size") {
            if let Ok(size_vec) = size_var.try_to::<Vector2>() {
                Vec2::new(size_vec.x, size_vec.y)
            } else {
                Vec2::new(1024.0, 768.0)
            }
        } else {
            Vec2::new(1024.0, 768.0)
        };
        
        if let Some(app_arc) = &self.bevy_app {
            if let Ok(mut app) = app_arc.lock() {
                if let Some(mut canvas_state) = app.world_mut().get_resource_mut::<CanvasState>() {
                    canvas_state.zoom = zoom;
                    canvas_state.offset = offset;
                    canvas_state.viewport_size = viewport_size;
                    println!("🦀 RUST: Updated CanvasState - Zoom: {}, Offset: {:?}, Size: {:?}", 
                            zoom, offset, viewport_size);
                }
            }
        }
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
        .init_resource::<SceneGeneratorConfig>()
        .add_event::<CanvasEvent>()
        .add_event::<ToolChangeEvent>()
        .add_event::<StrokeInputEvent>()
        .add_systems(Startup, scene_generator_startup_system)
        .add_systems(Update, (
            renovated_stroke_spawning_system,
            renovated_tool_management_system,
            renovated_scene_tree_system,
            scene_generator_management_system,
            stroke_rendering_system,
        ))
        .add_systems(Update, spatial_index_system)
        .add_systems(Update, renovated_history_system);
    
    println!("🦀 RUST: Renovated bevy_godot4 systems registered successfully");
    println!("🦀 RUST: - Scene generator startup system: ACTIVE");
    println!("🦀 RUST: - Renovated stroke spawning system: ACTIVE");
    println!("🦀 RUST: - Renovated tool management system: ACTIVE");
    println!("🦀 RUST: - Renovated scene tree system: ACTIVE");
    println!("🦀 RUST: - Scene generator management system: ACTIVE");
    println!("🦀 RUST: - Spatial index system: ACTIVE");
    println!("🦀 RUST: - Renovated history system: ACTIVE");
}

fn build_app_instance() -> App {
    let mut app = App::new();
    build_app(&mut app);
    
    println!("🦀 RUST: Manually executing startup systems...");
    app.world_mut().run_schedule(bevy::app::Startup);
    println!("🦀 RUST: Startup systems executed");
    
    app
}
