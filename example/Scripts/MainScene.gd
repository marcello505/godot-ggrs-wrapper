extends Node2D

var local_handle: int
var remote_handle: int

var frames_to_skip: int = 0

var game_started: bool = false


### STARTING A SESSION


func start_game(hosting: bool):
	if(hosting):
		$GodotGGRS.create_session(7070, 2) # Port 7070, 2 max players
		local_handle = $GodotGGRS.add_local_player()
		remote_handle = $GodotGGRS.add_remote_player("127.0.0.1:7071")
	else:
		$GodotGGRS.create_session(7071, 2) # Port 7071, 2 max players
		remote_handle = $GodotGGRS.add_remote_player("127.0.0.1:7070")
		local_handle = $GodotGGRS.add_local_player()

	$GodotGGRS.set_callback_node(self) # Set the node which will implement the callback methods
	$GodotGGRS.set_frame_delay(2, local_handle) # Set personal frame_delay, works only for local_handles.
	$GodotGGRS.start_session() #Start listening for a session.
	$Host.visible = false
	$Join.visible = false
	$Waiting.visible = true
	game_started = true


### ADVANCING FRAMES


func _process(_delta):
	if game_started:
		$GodotGGRS.poll_remote_clients() # GGRS needs to periodically process UDP requests and such, sticking it in \_process() works nicely since it's only called on idle.

func _physics_process(_delta):
	if $GodotGGRS.is_running(): # This will return true when all players and spectators have joined and have been synched.
		if $Waiting.visible:
			$Waiting.visible = false
		
		var events = $GodotGGRS.get_events()
		for item in events:
			match item[0]:
				"WaitRecommendation":
					frames_to_skip += item[1]	
		
		if frames_to_skip:
			frames_to_skip -= 1
			return
		
		$GodotGGRS.advance_frame(local_handle, raw_input_to_int("con1")) # raw_input_to_int is a method that parses InputActions that start with "con1" into a integer.
		var net_stats: Array = $GodotGGRS.get_network_stats(remote_handle)
		$NetStats.text = "Send queue len : %f\nPing : %f\nKbps sent : %f\nLocal frames behind : %f\nRemote frames behind : %f" % net_stats

func raw_input_to_int(prefix: String)->int:
	# This method is how i parse InputActions into an int, but as long as it's an int it doesn't matter how it's parsed.
	var result := 0;
	if(Input.is_action_pressed(prefix + "_left")): #The action it checks here would be "con1_left" if the prefix is set to "con1"
		result |= 1
	if(Input.is_action_pressed(prefix + "_right")):
		result |= 2
	if(Input.is_action_pressed(prefix + "_up")):
		result |= 4
	if(Input.is_action_pressed(prefix + "_down")):
		result |= 8
	return result;
	
	
### GGRS CALLBACKS


func ggrs_advance_frame(inputs: Array):
	# inputs is an array of input data indexed by handle.
	# input_data itself is also an array with the following: [frame: int, size: int, inputs: int]
	# frame can be used as a sanity check, size is used internally to properly slice the buffer of bytes and inputs is the int we created in our previous step.
	var net1_inputs := 0;
	var net2_inputs := 0;
	if(local_handle < remote_handle):
		net1_inputs = inputs[local_handle][2]
		net2_inputs = inputs[remote_handle][2]
	else:
		net1_inputs = inputs[remote_handle][2]
		net2_inputs = inputs[local_handle][2]
	int_to_raw_input("net1", net1_inputs) # Player objects check for InputActions that aren't bound to any controller.
	int_to_raw_input("net2", net2_inputs) # Player objects check for InputActions that aren't bound to any controller.
	_handle_player_frames()

func ggrs_load_game_state(frame: int, buffer: PoolByteArray, checksum: int):
	var state : Dictionary = bytes2var(buffer);
	$P1.load_state(state.get("P1", {}))
	$P2.load_state(state.get("P2", {}))

func ggrs_save_game_state(frame: int)->PoolByteArray: # frame parameter can be used as a sanity check (making sure it matches your internal frame counter).
	var save_state = {}
	save_state["P1"] = $P1.save_state()
	save_state["P2"] = $P2.save_state()
	return var2bytes(save_state);

func int_to_raw_input(prefix: String, inputs: int):
	_set_action(prefix + "_left", inputs & 1)
	_set_action(prefix + "_right", inputs & 2)
	_set_action(prefix + "_up", inputs & 4)
	_set_action(prefix + "_down", inputs & 8)

func _set_action(action: String, pressed: bool):
	if(pressed):
		Input.action_press(action)
	else:
		Input.action_release(action)


### GAMEPLAY HANDLING


func _handle_player_frames():
	if Input.is_action_pressed("net1_up"):
		$P1.up()
	if Input.is_action_pressed("net1_down"):
		$P1.down()
	if Input.is_action_pressed("net1_right"):
		$P1.right()
	if Input.is_action_pressed("net1_left"):
		$P1.left()
		
	if Input.is_action_pressed("net2_up"):
		$P2.up()
	if Input.is_action_pressed("net2_down"):
		$P2.down()
	if Input.is_action_pressed("net2_right"):
		$P2.right()
	if Input.is_action_pressed("net2_left"):
		$P2.left()
		
		
