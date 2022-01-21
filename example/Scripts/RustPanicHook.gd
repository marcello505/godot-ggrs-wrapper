extends Node

func rust_panic_hook(error_msg: String) -> void:
	assert(false, error_msg)
