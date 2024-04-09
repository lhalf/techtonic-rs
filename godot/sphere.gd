extends MeshInstance3D


# Called when the node enters the scene tree for the first time.
func _ready():
	draw_mesh()

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func draw_mesh():
	var mesh = ArrayMesh.new()
	mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, SphereMesh.new().get_mesh_arrays())
	#print(SphereMesh.new().get_mesh_arrays())
	var mdt = MeshDataTool.new()
	mdt.create_from_surface(mesh, 0)
	var material = StandardMaterial3D.new()
	material.set_albedo(Color.RED)
	mdt.set_material(material)
	for i in range(mdt.get_vertex_count()):
		var vertex = mdt.get_vertex(i)    # In this example we extend the mesh by one unit, which results in separated faces as it is flat shaded.    
		vertex += mdt.get_vertex_normal(i)    # Save your change.    
		mdt.set_vertex(i, vertex)
		mdt.set_vertex_color(i, Color(randf_range(0,1), randf_range(0,1), randf_range(0,1)))
	mesh.clear_surfaces()
	mdt.commit_to_surface(mesh)
	var mi = MeshInstance3D.new()
	mi.mesh = mesh
	add_child(mi)
	
	
	
