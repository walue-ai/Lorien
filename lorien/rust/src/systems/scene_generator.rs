use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;
use crate::{
    components::{Stroke, StrokeType},
    resources::StrokeResources,
};

#[derive(Component)]
pub struct GeneratedScene {
    pub scene_type: String,
    pub color: godot::builtin::Color,
    pub size: f32,
    pub configured: bool,
}

#[derive(Resource)]
pub struct SceneGeneratorConfig {
    pub enabled: bool,
    pub generation_count: u32,
    pub generated: bool,
}

impl Default for SceneGeneratorConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            generation_count: 5,
            generated: false,
        }
    }
}

pub fn scene_generator_startup_system(
    mut commands: Commands,
    stroke_resources: Res<StrokeResources>,
    mut generator_config: ResMut<SceneGeneratorConfig>,
) {
    if !generator_config.enabled || generator_config.generated {
        return;
    }

    println!("🦀 RUST: scene_generator_startup_system - Generating {} sample rectangles", generator_config.generation_count);

    let colors = [
        godot::builtin::Color::RED,
        godot::builtin::Color::GREEN,
        godot::builtin::Color::BLUE,
        godot::builtin::Color::YELLOW,
        godot::builtin::Color::MAGENTA,
    ];

    let sizes = [15.0, 25.0, 35.0, 45.0, 55.0];
    
    for i in 0..generator_config.generation_count {
        let color = colors[i as usize % colors.len()];
        let size = sizes[i as usize % sizes.len()];
        
        let rectangle_entity = commands.spawn((
            GodotScene::from_resource(stroke_resources.brush_stroke_scene.clone()),
            Stroke::new(StrokeType::Rectangle, color, size),
            GeneratedScene {
                scene_type: "rectangle".to_string(),
                color,
                size,
                configured: false,
            },
        )).id();

        println!("🦀 RUST: Generated rectangle entity {:?} - Color: {:?}, Size: {}", 
                rectangle_entity, color, size);
    }

    generator_config.generated = true;
    println!("🦀 RUST: Scene generation completed - {} rectangles spawned", generator_config.generation_count);
}

pub fn scene_generator_management_system(
    mut _scene_tree: SceneTreeRef,
    mut generated_scenes: Query<(Entity, &mut ErasedGd, &mut GeneratedScene), With<GeneratedScene>>,
) {
    for (entity, mut erased_gd, mut generated_scene) in generated_scenes.iter_mut() {
        if generated_scene.configured {
            continue;
        }
        if let Some(mut scene_node) = erased_gd.try_get::<godot::classes::Node2D>() {
            let x_pos = (entity.index() as f32 * 100.0) % 800.0;
            let y_pos = (entity.index() as f32 * 80.0) % 600.0;
            
            scene_node.set_position(Vector2::new(x_pos, y_pos));
            println!("🦀 RUST: Setting rectangle position to ({}, {})", x_pos, y_pos);
            
            if let Some(line2d) = scene_node.get_node_or_null("Line2D") {
                if let Ok(mut line2d_node) = line2d.try_cast::<godot::classes::Line2D>() {
                    line2d_node.set_default_color(generated_scene.color);
                    line2d_node.set_width(generated_scene.size);
                    
                    let rect_size = generated_scene.size * 3.0;
                    let points = PackedVector2Array::from(&[
                        Vector2::new(0.0, 0.0),
                        Vector2::new(rect_size, 0.0),
                        Vector2::new(rect_size, rect_size),
                        Vector2::new(0.0, rect_size),
                        Vector2::new(0.0, 0.0),
                    ]);
                    line2d_node.set_points(&points);
                }
            }
            
            generated_scene.configured = true;
            println!("🦀 RUST: Configured generated rectangle at position ({}, {}) with color {:?}", 
                    x_pos, y_pos, generated_scene.color);
            println!("🦀 RUST: Rectangle parent: {:?}", scene_node.get_parent().map(|p| p.get_name()));
        }
    }
}
