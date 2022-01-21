extends Node2D

export var speed: int = 10

func _ready():
	$Name.text = name

func save_state() -> Vector2:
	return position
	
func load_state(state: Vector2):
	position = state

func up():
	position.y -= speed
	
func down():
	position.y += speed
	
func right():
	position.x += speed

func left():
	position.x -= speed
	 
