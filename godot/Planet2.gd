extends Planet
var m = MeshInstance3D.new()
var mesh_texture = preload('res://texture.tres')

@export var change_mesh : bool = false

func _ready():
	draw_mesh(square())

func _process(delta):
	
	if change_mesh == true:
		change_mesh = false
		change_random_cell()


func draw_mesh(vertices: PackedVector3Array):
	var arrays = []
	arrays.resize(Mesh.ARRAY_MAX)
	arrays[Mesh.ARRAY_VERTEX] = vertices
	
	arrays = fake_grid_arrays()
	# Create the Mesh.
	var arr_mesh = ArrayMesh.new()
	arr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)
	m.mesh = arr_mesh
	m.mesh.surface_set_material(0, mesh_texture)
	
	add_child(m)

func fake_grid_arrays(width: int = 4, height: int = 2) -> Array:
	var output: Array = []
	output.resize(Mesh.ARRAY_MAX)
	var verticies = []
	var UVs = []
	
	for z in height:
		for x in width:
			var colour: float = randi_range(0,10) / 10.0
			for i in 6:
				UVs.push_back(Vector2(colour, 0))
			var four_corners: Array[Vector3] = []
			four_corners.push_back(Vector3(x, 0, z))
			four_corners.push_back(four_corners[0] + Vector3(1, 0, 0))
			four_corners.push_back(four_corners[0] + Vector3(1, 0, 1))
			four_corners.push_back(four_corners[0] + Vector3(0, 0, 1))
			var triangle_1 = [four_corners[0], four_corners[1], four_corners[3]]
			var triangle_2 = [four_corners[1], four_corners[2], four_corners[3]]
			verticies.append_array(triangle_1)
			verticies.append_array(triangle_2)
	
	output[Mesh.ARRAY_VERTEX] = PackedVector3Array(verticies)
	output[Mesh.ARRAY_TEX_UV] = PackedVector2Array(UVs)
	return output

func change_random_cell():
	change_cell(randi_range(0,7), randi_range(0,10) / 10.0)

func change_cell(cell_number: int, updated_cell_uv: float):
	var new_arrays = m.mesh.surface_get_arrays(0)
	for i in range(0,6):
		new_arrays[Mesh.ARRAY_TEX_UV][cell_number * 6 + i] = Vector2(updated_cell_uv, 0)
	
	var arr_mesh = ArrayMesh.new()
	arr_mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, new_arrays)
	m.mesh = arr_mesh
	m.mesh.surface_set_material(0, mesh_texture)