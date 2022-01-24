use crate::*;
use ggrs::{PlayerHandle, SyncTestSession};

/// A Godot implementation of [`SyncTestSession`]
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRSSyncTestSession {
    sess: Option<SyncTestSession>,
    callback_node: Option<Ref<Node>>,
}

impl GodotGGRSSyncTestSession {
    fn new(_owner: &Node) -> Self {
        GodotGGRSSyncTestSession {
            sess: None,
            callback_node: None,
        }
    }
}

#[methods]
impl GodotGGRSSyncTestSession {
    //EXPORTED FUNCTIONS
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("GodotGGRSSyncTest _ready() called.");
    }

    /// Creates a [SyncTestSession],
    /// call this when you want to start setting up a `SyncTestSession` takes the total number of players, the check distance and the max prediction frames as parameters
    /// # Notes
    /// - Max prediction frames is the maximum number of frames GGRS will roll back. Every gamestate older than this is guaranteed to be correct if the players did not desync.
    /// - This value used to default to `8 frames`, but this has been made adjustable with `GGRS 0.7.0`
    #[export]
    pub fn create_new_session(
        &mut self,
        _owner: &Node,
        num_players: u32,
        check_distance: usize,
        max_pred: usize,
    ) {
        let input_size: usize = std::mem::size_of::<u32>();
        match SyncTestSession::new(num_players, input_size, max_pred, check_distance) {
            Ok(s) => self.sess = Some(s),
            Err(e) => godot_error!("{}", e),
        }
    }

    /// Deprecated method to create a [SyncTestSession]. Use [Self::create_new_session()] instead.
    #[deprecated(since = "0.5.0", note = "please use `create_new_session()` instead")]
    #[export]
    pub fn create_session(&mut self, _owner: &Node, num_players: u32, check_distance: usize) {
        self.create_new_session(_owner, num_players, check_distance, 8)
    }

    /// Sets [SyncTestSession::set_frame_delay()] of specified handle.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_frame_delay(
        &mut self,
        _owner: &Node,
        frame_delay: u32,
        player_handle: PlayerHandle,
    ) {
        match &mut self.sess {
            Some(s) => match s.set_frame_delay(frame_delay, player_handle) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// This function will advance the frame using an array of all the inputs given as a parameter (inputs are currently an int in Godot).
    /// Before using this function you have to set the callback node and make sure it has the following callback functions implemented
    /// - [CALLBACK_FUNC_SAVE_GAME_STATE]
    /// - [CALLBACK_FUNC_LOAD_GAME_STATE]
    /// - [CALLBACK_FUNC_SAVE_GAME_STATE]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    /// - Will print a [ERR_MESSAGE_NO_CALLBACK_NODE] error if a callback node has not been set
    #[export]
    pub fn advance_frame(&mut self, _owner: &Node, all_inputs: Vec<u32>) {
        let mut all_inputs_bytes = Vec::new();
        for i in all_inputs {
            all_inputs_bytes.push(Vec::from(i.to_be_bytes()));
        }

        match self.callback_node {
            Some(callback_node) => match &mut self.sess {
                Some(s) => match s.advance_frame(&all_inputs_bytes) {
                    Ok(requests) => {
                        ggrs_request_handlers::handle_requests(&callback_node, requests);
                    }
                    Err(e) => {
                        godot_error!("{}", e);
                    }
                },
                None => {
                    godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                }
            },
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_CALLBACK_NODE);
            }
        }
    }

    /// Calls and returns [SyncTestSession::max_prediction()].
    /// Will return a 0 if no session was made.
    /// # Errors
    /// - Will print an [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_max_prediction(&mut self, _owner: &Node) -> usize {
        match &mut self.sess {
            Some(s) => s.max_prediction(),
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                return 0;
            }
        }
    }

    /// Sets the callback node that will be called when using [Self::advance_frame()]
    #[export]
    pub fn set_callback_node(&mut self, _owner: &Node, callback: Ref<Node>) {
        self.callback_node = Some(callback);
    }
}
