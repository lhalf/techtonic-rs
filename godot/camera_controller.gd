extends Node3D

@export var sensitivity: float = 0.15
@export var clamp_angle: int = 55
@export var vertical_rotation: Node3D

func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	
func _input(event: InputEvent):
	if event is InputEventKey and event.is_action_pressed("exit"):
		get_tree().quit()
	
	if event is InputEventMouseMotion:
		self.rotation_degrees.y += (-event.relative.x * sensitivity)
		# clamp to avoid weird over headness
		vertical_rotation.rotation_degrees.x = clamp(vertical_rotation.rotation_degrees.x + -event.relative.y * sensitivity, -clamp_angle, clamp_angle)
