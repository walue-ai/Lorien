use bevy::prelude::*;
use bevy_godot4::prelude::*;

#[derive(Resource, Debug)]
pub struct RenovatedToolResources {
    pub tool_scenes: std::collections::HashMap<String, ErasedGdResource>,
    pub active_tool_entity: Option<Entity>,
}

impl Default for RenovatedToolResources {
    fn default() -> Self {
        use godot::classes::ResourceLoader;
        let mut resource_loader = ResourceLoader::singleton();
        let mut tool_scenes = std::collections::HashMap::new();
        
        tool_scenes.insert("brush".to_string(), ErasedGdResource::new(
            resource_loader.load("res://BrushStroke/BrushStroke.tscn").unwrap()
        ));
        
        Self { 
            tool_scenes,
            active_tool_entity: None,
        }
    }
}
