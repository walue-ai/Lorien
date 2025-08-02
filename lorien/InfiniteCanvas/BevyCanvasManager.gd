extends Node
class_name BevyCanvasManager

var bevy_app: BevyApp
var current_stroke_id: int = 0

signal stroke_created(stroke_data: Dictionary)
signal tool_changed(tool_type: int, size: float, color: Color)

func _ready():
	bevy_app = BevyApp.new()
	add_child(bevy_app)

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
	tool_changed.emit(tool_type, size, color)

func start_stroke(position: Vector2, pressure: float):
	pass

func add_stroke_point(position: Vector2, pressure: float):
	pass

func end_stroke():
	pass

func undo():
	pass

func redo():
	pass

func clear_canvas():
	pass

func get_strokes_in_region(min_pos: Vector2, max_pos: Vector2) -> Array:
	return []

func set_canvas_transform(zoom: float, offset: Vector2, viewport_size: Vector2):
	pass
