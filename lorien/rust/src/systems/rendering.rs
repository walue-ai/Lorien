use bevy::prelude::*;
use bevy_godot4::prelude::*;
use godot::prelude::*;
use crate::{
    components::{Stroke, StrokeVisual, Dirty},
    resources::CanvasState,
};

pub fn stroke_rendering_system(
    mut commands: Commands,
    strokes_query: Query<(Entity, &Stroke), (With<Dirty>, Without<StrokeVisual>)>,
    canvas_state: Res<CanvasState>,
) {
    for (entity, stroke) in strokes_query.iter() {
        if let Some(godot_node) = create_godot_line2d(stroke, &canvas_state) {
            commands.entity(entity).insert(StrokeVisual { godot_node });
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
