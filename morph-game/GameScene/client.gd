extends Node3D
class_name morph_client

@export var ab_player: AbstractPlayer
@export var map: GridMap
@export var pl_stats: PlayerStats

@onready var texture_face_champion: TextureRect = $Control/FaceNLevel/TextureRect

const DISTANCE_TO_BE_CONSIDERATE: float = 40.0

var me_actions: Array[Player.PossibleAction] = []
var other_players: Dictionary = {}
var udp = PacketPeerUDP.new()

func _on_receive_cl_action(act: Player.PossibleAction):
	me_actions.append(act)

func create_frequencial_call(fn: Callable, freq: float):
	var timer = Timer.new()
	timer.wait_time = freq
	timer.autostart = true
	timer.one_shot = false
	timer.connect("timeout", fn)
	add_child(timer)

func _on_ping_heart_beat():
	send_packet(HeartBeat.get_payload(ab_player.player))
	
func _on_ping_cl_action():
	if me_actions.size() == 0:
		return
	
	send_packet(ClientAction.get_payload(ab_player.player, me_actions))
	me_actions.clear()
	
func _enter_tree() -> void:
	self.ab_player.init(pl_stats)
	var confirm_btn: Button = $Control/DBG_EID/confirm
	confirm_btn.set_stats(pl_stats)
	var lbl: Label = $Control/FaceNLevel/Label
	lbl.text = pl_stats.champion.name

func _ready() -> void:
	ab_player.player.in_action.connect(_on_receive_cl_action)
	texture_face_champion.texture = pl_stats.champion.logo
	
	var err = udp.connect_to_host("127.0.0.1", 6000)
	if err:
		return
	
	create_frequencial_call(_on_ping_heart_beat, HeartBeat.get_freq())
	create_frequencial_call(_on_ping_cl_action, ClientAction.get_freq())

func send_packet(payload: PackedByteArray):
	if not udp:
		return
	var err = udp.put_packet(payload)
	
func get_distance_between(fcoords: Vector3, scoords: Vector3) -> float:
	return sqrt(
		pow(fcoords.x-scoords.x, 2)+
		pow(fcoords.z-scoords.z, 2)
	)

func show_other_player() -> void:
	for i in other_players:
		var gp_oplayer: Vector3 = Vector3(
			other_players[i].content.coords_x,
			other_players[i].content.coords_y,
			other_players[i].content.coords_z
		)
		var gp_mplayer: Vector3 = ab_player.player.global_position
		var distance_between_me_and_oplayer: float = get_distance_between(gp_mplayer, gp_oplayer)
		
		if other_players[i].first_spawn:
			self.add_child(other_players[i].player)
			other_players[i].player.global_position = other_players[i].player.global_position.lerp(gp_oplayer, 0.2)
			#other_players[i].player.body_mesh.rotation = other_players[i].content.rotation
			other_players[i].first_spawn = false
			continue
		
		if distance_between_me_and_oplayer > DISTANCE_TO_BE_CONSIDERATE:
			print("hide player ", other_players[i].player)
			other_players[i].player.hide()
			continue
		else:
			print("show player ", other_players[i].player)
			other_players[i].player.show()
			
		gp_oplayer.y = ab_player.player.global_position.y
		other_players[i].player.global_position = other_players[i].player.global_position.lerp(gp_oplayer, 0.2)
		#other_players[i].player.body_mesh.rotation = other_players[i].content.rotation
		print("move player ", other_players[i].player, " at ", other_players[i].player.global_position)
		print(" dis: ", distance_between_me_and_oplayer);
			
func _process(_delta: float) -> void:
	while udp.get_available_packet_count() > 0:
		var incoming_payload: PackedByteArray = udp.get_packet()
		var payload_string: String = incoming_payload.get_string_from_utf8()
		var splited_payload: PackedStringArray = payload_string.split("|")
		
		var mp: MorphPayload.spayload = MorphPayload.process_server_payload(splited_payload)
		var mp_type: MorphPayload.PayloadType = mp.type;
		match mp_type:
			MorphPayload.PayloadType.Neighbors:
				for p in mp.content:
					if other_players.has(p.id):
						other_players[p.id].content = p
					else:
						var ps_stats: PlayerStats = load("res://Player/Statistics.tres")
						var ps: PackedScene = load("res://Player/Champions/Perso2/perso_2.tscn")
						
						if ps.can_instantiate():
							var pm: Player = ps.instantiate() as Player
							pm.set_stats(ps_stats)
							pm.gridmap = ab_player.gridmap
							pm.dead_zone = ab_player.dead_zone
							pm.terrain = ab_player.terrain
							pm.set_not_me(true)
							pm.scale = $Player.scale
							
							other_players.set(p.id, {
								"content": p,
								"player": pm,
								"is_visible": false,
								"first_spawn": true
							});
				
			_:
				pass

func _physics_process(_delta: float) -> void:
	show_other_player()
