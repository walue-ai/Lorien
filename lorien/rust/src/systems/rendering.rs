use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;
use crate::{
    components::{Stroke, StrokeVisual, Dirty},
    resources::CanvasState,
    systems::scene_generator::GeneratedScene,
};

pub fn stroke_rendering_system(
    mut commands: Commands,
    strokes_query: Query<(Entity, &Stroke), (With<Dirty>, Without<StrokeVisual>)>,
    scene_generator_query: Query<(Entity, &Stroke, &GeneratedScene), (With<Dirty>, Without<StrokeVisual>)>,
    all_entities_query: Query<Entity>,
    dirty_entities_query: Query<Entity, With<Dirty>>,
    stroke_entities_query: Query<Entity, With<Stroke>>,
    generated_scene_entities_query: Query<Entity, With<GeneratedScene>>,
    canvas_state: Res<CanvasState>,
    mut scene_tree: SceneTreeRef,
) {
    println!("🦀 RUST: stroke_rendering_system running!");
    println!("🦀 RUST: Total entities: {}", all_entities_query.iter().count());
    println!("🦀 RUST: Entities with Dirty: {}", dirty_entities_query.iter().count());
    println!("🦀 RUST: Entities with Stroke: {}", stroke_entities_query.iter().count());
    println!("🦀 RUST: Entities with GeneratedScene: {}", generated_scene_entities_query.iter().count());
    println!("🦀 RUST: Found {} stroke entities and {} scene generator entities to process", 
             strokes_query.iter().count(), scene_generator_query.iter().count());
    
    for (entity, stroke) in strokes_query.iter() {
        println!("🦀 RUST: Processing stroke entity {:?} with {} points", entity, stroke.points.len());
        
        if let Some(godot_node) = create_godot_line2d(stroke, &canvas_state) {
            commands.entity(entity).insert(StrokeVisual { godot_node });
            println!("🦀 RUST: Added StrokeVisual component to entity {:?}", entity);
        } else {
            println!("🦀 RUST: Failed to create godot_node for entity {:?}", entity);
        }
    }
    
    for (entity, _stroke, generated_scene) in scene_generator_query.iter() {
        println!("🦀 RUST: Processing scene generator entity {:?} - Color: {:?}, Size: {}", 
                 entity, generated_scene.color, generated_scene.size);
        
        if let Some(godot_node) = create_rectangle_line2d(generated_scene, &canvas_state, &mut scene_tree) {
            commands.entity(entity).insert(StrokeVisual { godot_node });
            println!("🦀 RUST: Added StrokeVisual component to scene generator entity {:?}", entity);
        } else {
            println!("🦀 RUST: Failed to create rectangle node for entity {:?}", entity);
        }
    }
}

fn create_godot_line2d(stroke: &Stroke, _canvas_state: &CanvasState) -> Option<ErasedGd> {
    use godot::classes::Line2D;
    use godot::builtin::PackedVector2Array;
    use godot::builtin::Vector2;
    
    let mut line2d = Line2D::new_alloc();
    
    let mut points = PackedVector2Array::new();
    for point in &stroke.points {
        points.push(Vector2::new(point.x, point.y));
    }
    
    line2d.set_points(&points);
    line2d.set_default_color(stroke.color);
    line2d.set_width(stroke.size);
    
    Some(ErasedGd::new(line2d.upcast::<Node>()))
}

fn create_rectangle_line2d(generated_scene: &GeneratedScene, _canvas_state: &CanvasState, scene_tree: &mut SceneTreeRef) -> Option<ErasedGd> {
    use godot::classes::Line2D;
    use godot::builtin::PackedVector2Array;
    use godot::builtin::Vector2;
    use godot::prelude::Node;
    
    let mut line2d = Line2D::new_alloc();
    
    let half_size = generated_scene.size / 2.0;
    let mut points = PackedVector2Array::new();
    points.push(Vector2::new(-half_size, -half_size));
    points.push(Vector2::new(half_size, -half_size));
    points.push(Vector2::new(half_size, half_size));
    points.push(Vector2::new(-half_size, half_size));
    points.push(Vector2::new(-half_size, -half_size)); // Close the rectangle
    
    line2d.set_points(&points);
    line2d.set_default_color(generated_scene.color);
    line2d.set_width(2.0); // Fixed width for rectangles
    
    if let Some(mut root) = scene_tree.get().get_root() {
        if let Some(mut canvas) = root.get_node_or_null("Main/InfiniteCanvas") {
            canvas.add_child(&line2d.clone().upcast::<Node>());
            println!("🦀 RUST: Added rectangle Line2D to InfiniteCanvas");
        } else if let Some(mut main) = root.get_node_or_null("Main") {
            main.add_child(&line2d.clone().upcast::<Node>());
            println!("🦀 RUST: Added rectangle Line2D to Main node");
        } else {
            root.add_child(&line2d.clone().upcast::<Node>());
            println!("🦀 RUST: Added rectangle Line2D to root");
        }
    }
    
    Some(ErasedGd::new(line2d.upcast::<Node>()))
}
