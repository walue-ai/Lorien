use bevy::prelude::*;
use crate::{
    components::{Stroke, Dirty},
    resources::{ActiveTool, CanvasState},
    events::{StrokeInputEvent, CanvasEvent},
};

pub fn stroke_creation_system(
    mut commands: Commands,
    mut input_events: EventReader<StrokeInputEvent>,
    active_tool: Res<ActiveTool>,
    mut canvas_state: ResMut<CanvasState>,
    mut canvas_events: EventWriter<CanvasEvent>,
    mut current_stroke: Local<Option<Entity>>,
    mut strokes: Query<&mut Stroke>,
) {
    for event in input_events.read() {
        if event.is_start {
            println!("🦀 RUST: stroke_creation_system - Starting new stroke");
            println!("🦀 RUST: - Tool: {:?}, Color: {:?}, Size: {}", 
                active_tool.tool_type, active_tool.brush_color, active_tool.brush_size);
            println!("🦀 RUST: - Position: {:?}, Pressure: {}", event.position, event.pressure);
            
            let mut stroke = Stroke::new(
                active_tool.tool_type,
                active_tool.brush_color,
                active_tool.brush_size,
            );
            stroke.add_point(event.position, event.pressure);

            let entity = commands.spawn((
                stroke.clone(),
                Dirty,
            )).id();

            *current_stroke = Some(entity);
            canvas_state.stroke_counter += 1;
            
            println!("🦀 RUST: - Created stroke entity: {:?}, Total strokes: {}", 
                entity, canvas_state.stroke_counter);

            canvas_events.write(CanvasEvent::StrokeAdded {
                entity,
                stroke_data: (&stroke).into(),
            });
        } else if let Some(entity) = *current_stroke {
            if let Ok(mut stroke) = strokes.get_mut(entity) {
                stroke.add_point(event.position, event.pressure);
                commands.entity(entity).insert(Dirty);
                println!("🦀 RUST: stroke_creation_system - Added point to stroke {:?}, total points: {}", 
                    entity, stroke.points.len());
            }

            if event.is_end {
                println!("🦀 RUST: stroke_creation_system - Finished stroke {:?}", entity);
                *current_stroke = None;
            }
        }
    }
}

pub fn stroke_optimization_system(
    mut strokes: Query<(Entity, &mut Stroke), With<Dirty>>,
    mut commands: Commands,
) {
    for (entity, mut stroke) in strokes.iter_mut() {
        let original_points = stroke.points.len();
        
        if stroke.points.len() > 2 {
            optimize_stroke_points(&mut stroke);
            let optimized_points = stroke.points.len();
            let removed_points = original_points - optimized_points;
            
            println!("🦀 RUST: stroke_optimization_system - Optimized stroke {:?}", entity);
            println!("🦀 RUST: - Original points: {}, Optimized: {}, Removed: {}", 
                original_points, optimized_points, removed_points);
        }

        if let Some(bounds) = stroke.calculate_bounds() {
            commands.entity(entity).insert(bounds);
            println!("🦀 RUST: stroke_optimization_system - Calculated bounds for stroke {:?}: {:?}", 
                entity, bounds);
        }

        commands.entity(entity).remove::<Dirty>();
    }
}

fn optimize_stroke_points(stroke: &mut Stroke) {
    if stroke.points.len() < 3 {
        return;
    }

    let mut optimized_points = Vec::new();
    let mut optimized_pressures = Vec::new();

    optimized_points.push(stroke.points[0]);
    optimized_pressures.push(stroke.pressures[0]);

    let tolerance = 2.0;

    for i in 1..stroke.points.len() - 1 {
        let prev = stroke.points[i - 1];
        let curr = stroke.points[i];
        let next = stroke.points[i + 1];

        let distance_to_line = point_to_line_distance(curr, prev, next);

        if distance_to_line > tolerance {
            optimized_points.push(curr);
            optimized_pressures.push(stroke.pressures[i]);
        }
    }

    optimized_points.push(*stroke.points.last().unwrap());
    optimized_pressures.push(*stroke.pressures.last().unwrap());

    stroke.points = optimized_points;
    stroke.pressures = optimized_pressures;
}

fn point_to_line_distance(point: Vec2, line_start: Vec2, line_end: Vec2) -> f32 {
    let line_vec = line_end - line_start;
    let point_vec = point - line_start;
    
    let line_len_sq = line_vec.length_squared();
    if line_len_sq == 0.0 {
        return point_vec.length();
    }

    let t = (point_vec.dot(line_vec) / line_len_sq).clamp(0.0, 1.0);
    let projection = line_start + t * line_vec;
    (point - projection).length()
}
