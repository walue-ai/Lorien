use bevy::prelude::*;
use bevy_godot4::prelude::*;

#[derive(Resource, Debug)]
pub struct StrokeResources {
    pub brush_stroke_scene: ErasedGdResource,
}

impl Default for StrokeResources {
    fn default() -> Self {
        use godot::classes::ResourceLoader;
        let mut resource_loader = ResourceLoader::singleton();
        let brush_stroke_scene = ErasedGdResource::new(
            resource_loader.load("res://BrushStroke/BrushStroke.tscn").unwrap()
        );

        Self { brush_stroke_scene }
    }
}
