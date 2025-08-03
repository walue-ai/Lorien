extends Node
class_name BevyCanvasManager

var bevy_app: BevyApp
var current_stroke_id: int = 0

signal stroke_created(stroke_data: Dictionary)
signal tool_changed(tool_type: int, size: float, color: Color)
signal scene_tree_updated(operation: String, data: Dictionary)

func _on_bevy_stroke_created(stroke_data: Dictionary):
	print("🦀 RUST: Received stroke_created signal from Bevy ECS: ", stroke_data)
	stroke_created.emit(stroke_data)

func _ready():
	print("🎯 GODOT: BevyCanvasManager initializing with renovated bevy_godot4 architecture...")
	bevy_app = get_node("/root/BevyAppSingleton/LorienBevyManager")
	print("🎯 GODOT: BevyCanvasManager connected to BevyAppSingleton")
	
	if bevy_app.has_signal("stroke_created"):
		bevy_app.stroke_created.connect(_on_bevy_stroke_created)
		print("🦀 RUST: Connected to Bevy stroke_created signal")
	
	print("🦀 RUST: BevyCanvasManager initialization complete")

func create_stroke(points: Array, pressures: Array, color: Color, size: float, stroke_type: int) -> int:
	current_stroke_id += 1
	var stroke_data = {
		"id": current_stroke_id,
		"points": points,
		"pressures": pressures,
		"color": color,
		"size": size,
		"stroke_type": stroke_type
	}
	stroke_created.emit(stroke_data)
	return current_stroke_id

func update_tool(tool_type: int, size: float, color: Color, opacity: float = 1.0):
	print("🎯 GODOT: update_tool - Using renovated ErasedGd tool management")
	tool_changed.emit(tool_type, size, color)
	
	if bevy_app:
		var tool_data = {
			"tool_type": tool_type,
			"size": size,
			"color": color,
			"opacity": opacity
		}
		bevy_app.call("update_tool_state", tool_data)
		print("🎯 GODOT: Sent tool change event to renovated bevy_godot4 system")

func start_stroke(position: Vector2, pressure: float):
	print("🎯 GODOT: start_stroke - Using renovated GodotScene spawning")
	if bevy_app:
		var stroke_data = {
			"position": position,
			"pressure": pressure,
			"is_start": true,
			"is_end": false
		}
		bevy_app.call("spawn_stroke_scene", stroke_data)
		print("🎯 GODOT: Sent start stroke event to renovated bevy_godot4 system")

func add_stroke_point(position: Vector2, pressure: float):
	print("🎯 GODOT: add_stroke_point - Advanced ECS processing")
	if bevy_app:
		var stroke_data = {
			"position": position,
			"pressure": pressure,
			"is_start": false,
			"is_end": false
		}
		bevy_app.call("spawn_stroke_scene", stroke_data)
		print("🎯 GODOT: Sent stroke point event to renovated bevy_godot4 system")

func end_stroke():
	print("🎯 GODOT: end_stroke - Finalizing with bevy_godot4 systems")
	if bevy_app:
		var stroke_data = {
			"position": Vector2.ZERO,
			"pressure": 0.0,
			"is_start": false,
			"is_end": true
		}
		bevy_app.call("spawn_stroke_scene", stroke_data)
		print("🎯 GODOT: Sent end stroke event to renovated bevy_godot4 system")

func manage_scene_operation(operation: String, data: Dictionary):
	print("🎯 GODOT: manage_scene_operation - Using SceneTreeRef: ", operation)
	if bevy_app:
		bevy_app.call("manage_scene_tree", operation, data)
		scene_tree_updated.emit(operation, data)
		print("🎯 GODOT: Sent scene operation to renovated bevy_godot4 system")

func undo():
	print("🎯 GODOT: undo - Using renovated ECS history system")
	manage_scene_operation("undo", {})

func redo():
	print("🎯 GODOT: redo - Using renovated ECS history system")
	manage_scene_operation("redo", {})

func clear_canvas():
	print("🎯 GODOT: clear - Using advanced scene tree management")
	manage_scene_operation("clear", {})

func get_strokes_in_region(min_pos: Vector2, max_pos: Vector2) -> Array:
	print("🎯 GODOT: get_strokes_in_region - Using renovated spatial indexing")
	if bevy_app:
		var region_data = {
			"x": min_pos.x,
			"y": min_pos.y,
			"width": max_pos.x - min_pos.x,
			"height": max_pos.y - min_pos.y
		}
		var result = bevy_app.call("get_strokes_in_region", region_data)
		print("🎯 GODOT: Retrieved ", result.size() if result else 0, " strokes from renovated bevy_godot4 system")
		return result if result else []
	return []

func set_canvas_transform(zoom: float, offset: Vector2, viewport_size: Vector2):
	print("🦀 RUST: BevyCanvasManager.set_canvas_transform called - Zoom: ", zoom, " Offset: ", offset, " Size: ", viewport_size)
	if bevy_app:
		var transform_data = {
			"zoom": zoom,
			"offset": offset,
			"viewport_size": viewport_size
		}
		bevy_app.call("set_canvas_transform", transform_data)
		print("🦀 RUST: Sent canvas transform to Bevy ECS")
