extends CharacterBody3D

@export var sensitivity: float = 0.15
@export var clamp_angle: int = 55
@export var vertical_rotation: Node3D
@export var max_speed: int = 10
@export var acceleration: int = 2

func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)
	self.motion_mode = CharacterBody3D.MOTION_MODE_FLOATING
	
func _input(event: InputEvent):
	if event is InputEventKey and event.is_action_pressed("exit"):
		get_tree().quit()
	
	if event is InputEventMouseMotion:
		self.rotation_degrees.y += (-event.relative.x * sensitivity)
		# clamp to avoid weird over headness
		vertical_rotation.rotation_degrees.x = clamp(vertical_rotation.rotation_degrees.x + -event.relative.y * sensitivity, -clamp_angle, clamp_angle)

func _process(delta: float) -> void:
	var input = Input.get_vector("left", "right", "forward", "backward")
	var relative_input = Vector2(input.x, input.y).rotated(-self.global_rotation.y)
	if input.length() > 0:
		self.velocity.x = lerp(self.velocity.x, relative_input.x * max_speed, acceleration * delta)
		self.velocity.z = lerp(self.velocity.z, relative_input.y * max_speed, acceleration * delta)
	else:
		self.velocity.x = lerp(self.velocity.x, 0.0, acceleration * delta)
		self.velocity.z = lerp(self.velocity.z, 0.0, acceleration * delta)

func _physics_process(_delta: float) -> void:
	self.move_and_slide()
