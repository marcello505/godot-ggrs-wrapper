use crate::*;
use ggrs::{Frame, GGRSRequest, GameState, GameStateCell};

pub fn handle_requests(callback_node: &Ref<Node>, requests: Vec<GGRSRequest>) {
    for item in requests {
        match item {
            GGRSRequest::AdvanceFrame { inputs } => {
                ggrs_request_advance_fame(callback_node, inputs)
            }
            GGRSRequest::LoadGameState { cell, frame } => {
                ggrs_request_load_game_state(callback_node, cell, frame)
            }
            GGRSRequest::SaveGameState { cell, frame } => {
                ggrs_request_save_game_state(callback_node, cell, frame);
            }
        }
    }
}

pub fn ggrs_request_advance_fame(callback_node: &Ref<Node>, inputs: Vec<ggrs::GameInput>) {
    //Parse parameter inputs in a way that godot can handle then call the callback method
    let node = unsafe { callback_node.assume_safe() };
    let mut godot_array: Vec<Variant> = Vec::new();
    for i in inputs {
        let result = (
            i.frame,
            i.size,
            u32::from_be_bytes(
                i.buffer[..i.size]
                    .try_into()
                    .expect("Slice size is too big or too small to convert into u32"),
            ),
        )
            .to_variant();
        godot_array.push(result);
    }
    unsafe { node.call(CALLBACK_FUNC_ADVANCE_FRAME, &[godot_array.to_variant()]) };
}

pub fn ggrs_request_load_game_state(callback_node: &Ref<Node>, cell: GameStateCell, _frame: Frame) {
    //Unpack the cell and have over it's values to godot so it can handle it.
    let node = unsafe { callback_node.assume_safe() };
    let game_state = cell.load();
    let frame = game_state.frame.to_variant();
    let buffer = ByteArray::from_vec(game_state.data.unwrap_or_default()).to_variant();
    let checksum = game_state.checksum.to_variant();
    unsafe { node.call(CALLBACK_FUNC_LOAD_GAME_STATE, &[frame, buffer, checksum]) };
}

pub fn ggrs_request_save_game_state(callback_node: &Ref<Node>, cell: GameStateCell, frame: Frame) {
    //Store current cell for later use
    let node = unsafe { callback_node.assume_safe() };
    let state: Variant = unsafe { node.call(CALLBACK_FUNC_SAVE_GAME_STATE, &[frame.to_variant()]) };
    let state_bytes = ByteArray::from_variant(&state).unwrap_or_default();
    let mut state_bytes_vec = Vec::new();
    for i in 0..state_bytes.len() {
        state_bytes_vec.push(state_bytes.get(i));
    }
    let result = GameState::new(frame, Some(state_bytes_vec));
    cell.save(result);
}
