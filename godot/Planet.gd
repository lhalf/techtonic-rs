extends Planet

@onready var mesh_texture = preload('res://texture.tres')
var mesh_arrays: Array = []
@export var change_mesh : bool = false

func _enter_tree():
	draw_mesh(vertices(), Vector2(randi_range(0,10) / 10.0, 0))

func _process(_delta):
	if change_mesh == true:
		change_mesh = false
		change_random_cell()


func draw_mesh(vertices: PackedVector3Array, colour: Vector2):
	mesh_arrays.resize(Mesh.ARRAY_MAX) # setting unused stuff to zero
	mesh_arrays[Mesh.ARRAY_VERTEX] = vertices
	var UVs = []
	UVs.resize(vertices.size())
	UVs.fill(colour)
	mesh_arrays[Mesh.ARRAY_TEX_UV] = PackedVector2Array(UVs)

	# Create the Mesh.
	var arr_mesh = ArrayMesh.new()
	arr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, mesh_arrays)
	
	mesh = arr_mesh
	mesh.surface_set_material(0, mesh_texture)

func change_random_cell():
	change_cell(0, randi_range(0,10) / 10.0)

func change_cell(cell_number: int, updated_cell_uv: float):
	for i in range(0,6):
		mesh_arrays[Mesh.ARRAY_TEX_UV][cell_number * 6 + i] = Vector2(updated_cell_uv, 0)
	
	var arr_mesh = ArrayMesh.new()
	arr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, mesh_arrays)
	mesh = arr_mesh
	mesh.surface_set_material(0, mesh_texture)
