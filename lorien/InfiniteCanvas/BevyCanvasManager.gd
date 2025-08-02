extends Node
class_name BevyCanvasManager

var bevy_app: BevyApp
var current_stroke_id: int = 0

signal stroke_created(stroke_data: Dictionary)
signal tool_changed(tool_type: int, size: float, color: Color)

func _on_bevy_stroke_created(stroke_data: Dictionary):
	print("🦀 RUST: Received stroke_created signal from Bevy ECS: ", stroke_data)
	stroke_created.emit(stroke_data)

func _ready():
	print("🦀 RUST: BevyCanvasManager initializing...")
	bevy_app = BevyApp.new()
	add_child(bevy_app)
	print("🦀 RUST: BevyApp created and added to scene tree")
	
	# Connect Bevy signals if available
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
	print("🦀 RUST: BevyCanvasManager.update_tool called - Type: ", tool_type, " Size: ", size, " Color: ", color)
	tool_changed.emit(tool_type, size, color)
	
	if bevy_app:
		var tool_data = {
			"tool_type": tool_type,
			"size": size,
			"color": color,
			"opacity": opacity
		}
		bevy_app.call("send_tool_change_event", tool_data)
		print("🦀 RUST: Sent tool change event to Bevy ECS")

func start_stroke(position: Vector2, pressure: float):
	print("🦀 RUST: BevyCanvasManager.start_stroke called - Position: ", position, " Pressure: ", pressure)
	if bevy_app:
		# Send stroke input event to Bevy ECS
		var event_data = {
			"position": position,
			"pressure": pressure,
			"is_start": true,
			"is_end": false
		}
		bevy_app.call("send_stroke_input_event", event_data)
		print("🦀 RUST: Sent start stroke event to Bevy ECS")

func add_stroke_point(position: Vector2, pressure: float):
	print("🦀 RUST: BevyCanvasManager.add_stroke_point called - Position: ", position, " Pressure: ", pressure)
	if bevy_app:
		# Send stroke input event to Bevy ECS
		var event_data = {
			"position": position,
			"pressure": pressure,
			"is_start": false,
			"is_end": false
		}
		bevy_app.call("send_stroke_input_event", event_data)
		print("🦀 RUST: Sent stroke point event to Bevy ECS")

func end_stroke():
	print("🦀 RUST: BevyCanvasManager.end_stroke called")
	if bevy_app:
		# Send stroke input event to Bevy ECS
		var event_data = {
			"position": Vector2.ZERO,
			"pressure": 0.0,
			"is_start": false,
			"is_end": true
		}
		bevy_app.call("send_stroke_input_event", event_data)
		print("🦀 RUST: Sent end stroke event to Bevy ECS")

func undo():
	print("🦀 RUST: BevyCanvasManager.undo called")
	if bevy_app:
		bevy_app.call("send_undo_event")
		print("🦀 RUST: Sent undo event to Bevy ECS")

func redo():
	print("🦀 RUST: BevyCanvasManager.redo called")
	if bevy_app:
		bevy_app.call("send_redo_event")
		print("🦀 RUST: Sent redo event to Bevy ECS")

func clear_canvas():
	print("🦀 RUST: BevyCanvasManager.clear_canvas called")
	if bevy_app:
		bevy_app.call("send_clear_event")
		print("🦀 RUST: Sent clear canvas event to Bevy ECS")

func get_strokes_in_region(min_pos: Vector2, max_pos: Vector2) -> Array:
	print("🦀 RUST: BevyCanvasManager.get_strokes_in_region called - Min: ", min_pos, " Max: ", max_pos)
	if bevy_app:
		var region_data = {
			"min_pos": min_pos,
			"max_pos": max_pos
		}
		var result = bevy_app.call("get_strokes_in_region", region_data)
		print("🦀 RUST: Retrieved ", result.size() if result else 0, " strokes from Bevy ECS")
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
