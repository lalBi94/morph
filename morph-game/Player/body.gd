extends CharacterBody3D
class_name Player

@export var gridmap: GridMap
@export var dead_zone: Area3D
@export var terrain: Terrain3D
@export var is_not_me: bool

@onready var animation_tree: AnimationTree = $AnimationTree
@onready var body_mesh: MeshInstance3D = $Armature/Skeleton3D/Mesh
@onready var camera_arm: SpringArm3D = $Camera
@onready var camera_of_arm: Camera3D = $Camera/Camera3D

enum under_feet {
	nothing = -1,
	regular = 0,
	lava = 5
}

const ZOOM_SPEED_REDUCER: float = 0.4
const ZOOM_PRIMARY_SCALE_FACTOR: float = 1.28233333333
const ZOOM_ADD: float = 1.1*ZOOM_PRIMARY_SCALE_FACTOR
var is_zooming = false

var deltaplane_ref: Node3D = null
var is_gliding: bool = false

var is_sprinting: bool = false
var last_position_in_floor = self.global_position

var chest_ref: Node3D = null
var is_dead: bool = false

var stats: PlayerStats

signal fall()

func set_not_me(response: bool) -> void:
	self.is_not_me = response

func set_stats(pstats: PlayerStats) -> void:
	self.stats = pstats

func _input(event):
	if event is InputEventMouseMotion:
		var mouse_pos: Vector2 = event.position

		var from: Vector3 = camera_of_arm.project_ray_origin(mouse_pos)
		var dir: Vector3 = camera_of_arm.project_ray_normal(mouse_pos)
		var to: Vector3 = from + dir * 20000.0
		
		var query:= PhysicsRayQueryParameters3D.create(from, to)
		query.collision_mask = 1
		
		var hit: Dictionary = get_world_3d().direct_space_state.intersect_ray(query)
		var target: Vector3
		if not hit.is_empty():
			target = hit.position
		else:
			var plane := Plane(Vector3.UP, global_position.y)
			var denom := plane.normal.dot(dir)
			if abs(denom) < 1e-6:
				return
			var t := -(plane.normal.dot(from) + plane.d) / denom
			if t < 0.0:
				return
			target = from + dir * t
		
		var pos: Vector3 = global_position
		
		var d: Vector3 = target - pos
		d.y = 0.0
		if d.length_squared() < 1e-8:
			return
		
		var yaw: float = atan2(d.x, d.z)
		
		body_mesh.rotation.y = yaw
		
		if deltaplane_ref:
			deltaplane_ref.rotation.y = yaw

func spawn_chest() -> Node3D:
	var inst: Node3D = stats.chest_mesh.instantiate() as Node3D
	add_child(inst)
	inst.global_position = self.global_position
	inst.rotation.y = body_mesh.rotation.y
	inst.scale = inst.scale * 3
	return inst

func spawn_deltaplane() -> Node3D:
	var inst: Node3D = stats.deltaplane_mesh.instantiate() as Node3D
	add_child(inst)
	inst.global_position = self.global_position + Vector3(0,7,0)
	inst.rotation.y = body_mesh.rotation.y
	inst.scale = inst.scale * 50
	return inst

func cell_under_player(max_dist := 2.0) -> int:
	var from := global_position + Vector3.UP * 0.3
	var to   := from + Vector3.DOWN * max_dist
	
	var query := PhysicsRayQueryParameters3D.create(from, to)
	query.exclude = [self]
	query.collide_with_bodies = true
	query.collide_with_areas = false
	query.collision_mask = 1
	
	var hit := get_world_3d().direct_space_state.intersect_ray(query)
	if hit.is_empty():
		return under_feet.nothing
		
		
	# Décalage vers l'intérieur (epsilon)
	var p : Vector3 = hit.position - hit.normal
	var cell : Vector3i = gridmap.local_to_map(gridmap.to_local(p))
	var item_id : int = gridmap.get_cell_item(cell)

	return item_id

func _on_fall() -> void:
	stats.fall()
	is_dead = true

func _ready() -> void:
	self.fall.connect(_on_fall)
	Input.mouse_mode = Input.MOUSE_MODE_VISIBLE
	stats.reset_runtime()

func _process(_delta: float) -> void:
	# Box Respawing last position
	if is_on_floor():
		dead_zone.set("respawn_box_position", self.global_position)
		
	if stats.hp <= 0.0:
		is_dead = true

func _physics_process(delta: float) -> void:
	# dead condition
	if is_dead and not chest_ref:
		chest_ref = spawn_chest()
		body_mesh.hide()
	
	if is_dead and chest_ref:
		return
	
	# Sprint
	if Input.is_action_just_pressed("morph_sprint"):
		is_sprinting = true
	elif Input.is_action_just_released("morph_sprint"):
		is_sprinting = false
		
	# Send is on floor to animator automata
	animation_tree.set("is_on_floor", self.is_on_floor())
	
	# Send speed to animator automata
	animation_tree.set("speed", self.velocity.length())
	
	# Add the gravity.
	if not is_on_floor() and not is_gliding:
		velocity.y -= 35 * delta
	elif is_gliding:
		velocity.y -= 5 * delta

	# Mouvements
	if !is_not_me:
		var input_dir := Input.get_vector("ui_left", "ui_right", "ui_up", "ui_down")
		var direction := (transform.basis * Vector3(-input_dir.x, 0, -input_dir.y)).normalized()

		var speed := stats.move_speed
		if is_zooming:
			speed *= ZOOM_SPEED_REDUCER
		if is_sprinting:
			speed *= 2

		if direction.length() > 0.0:
			velocity.x = direction.x * speed
			velocity.z = direction.z * speed
		else:
			var decel := stats.move_speed
			velocity.x = move_toward(velocity.x, 0.0, decel)
			velocity.z = move_toward(velocity.z, 0.0, decel)

	# Unzoom
	if !is_not_me && Input.is_action_just_pressed("morph_zoom"):
		camera_arm.position *= ZOOM_ADD
		is_zooming = true
	if !is_not_me && Input.is_action_just_released("morph_zoom"):
		camera_arm.position /= ZOOM_ADD
		is_zooming = false

	# Handle jump.
	if !is_not_me && Input.is_action_just_pressed("ui_accept") and is_on_floor():
		velocity.y = stats.jump_velocity + 9

	if is_gliding and body_mesh:
		deltaplane_ref.rotation.y = body_mesh.rotation.y

	if is_gliding and velocity.y < -8.0:
		velocity.y = move_toward(velocity.y, -8.0, 20.0 * delta)

	# deltaplane
	if !is_not_me && Input.is_action_just_pressed("ui_accept") and !self.is_on_floor():
		self.deltaplane_ref = self.spawn_deltaplane()
		is_gliding = true
	
	if !is_not_me && Input.is_action_just_released("ui_accept") and is_gliding:
		self.deltaplane_ref.queue_free()
		is_gliding = false
		
	if self.is_on_floor() and is_gliding:
		is_gliding = false
		self.deltaplane_ref.queue_free()

	# Detect block under
	var block_under: int = cell_under_player(129.0)
	
	if self.is_on_floor() :
		match block_under :
			under_feet.lava:
				stats.walk_on_lava()
			_:
				pass

	move_and_slide()
