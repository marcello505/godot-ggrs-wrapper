//This function can make a hash out of binary data.
pub const ERR_MESSAGE_NO_SESSION_MADE: &str = "No session was made.";
pub const ERR_MESSAGE_NO_CALLBACK_NODE: &str = "No callback node was specified.";
pub const CALLBACK_FUNC_SAVE_GAME_STATE: &str = "ggrs_save_game_state";
pub const CALLBACK_FUNC_LOAD_GAME_STATE: &str = "ggrs_load_game_state";
pub const CALLBACK_FUNC_ADVANCE_FRAME: &str = "ggrs_advance_frame";

pub fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for index in 0..data.len() {
        sum1 = (sum1 + data[index] as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}
