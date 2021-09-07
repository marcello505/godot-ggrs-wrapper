use crate::helper_functions::*;
use gdnative::prelude::*;
use ggrs::*;
use std::convert::TryInto;
use std::option::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRSP2PSpectatorSession {
    sess: Option<P2PSpectatorSession>,
    callback_node: Option<Ref<Node>>,
}

impl GodotGGRSP2PSpectatorSession {
    fn new(_owner: &Node) -> Self {
        GodotGGRSP2PSpectatorSession {
            sess: None,
            callback_node: None,
        }
    }
}

#[methods]
impl GodotGGRSP2PSpectatorSession {
    //EXPORTED FUNCTIONS
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("GodotGGRSP2PSpectatorSession _ready() called.");
    }

    #[export]
    fn create_session(
        &mut self,
        _owner: &Node,
        local_port: u16,
        num_players: u32,
        host_addr: String,
    ) {
        let input_size: usize = std::mem::size_of::<u32>();
        let host_addr_object: std::net::SocketAddr = host_addr.parse().unwrap();
        match start_p2p_spectator_session(num_players, input_size, local_port, host_addr_object) {
            Ok(s) => self.sess = Some(s),
            Err(e) => godot_error!("{}", e),
        }
    }

    #[export]
    fn is_running(&mut self, _owner: &Node) -> bool {
        match &mut self.sess {
            Some(s) => s.current_state() == SessionState::Running,
            None => false,
        }
    }

    #[export]
    fn start_session(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.start_session() {
                Ok(_) => godot_print!("Started GodotGGRS session"),
                Err(e) => {
                    godot_error!("{}", e);
                    panic!()
                }
            },
            None => {
                godot_error!("No session was made.")
            }
        }
    }

    #[export]
    fn receive_callback_node(&mut self, _owner: &Node, callback: Ref<Node>) {
        self.callback_node = Some(callback);
    }

    #[export]
    fn advance_frame(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.advance_frame() {
                Ok(requests) => {
                    self.handle_requests(requests);
                }
                Err(e) => {
                    godot_error!("{}", e);
                }
            },
            None => {
                godot_error!("No session was made");
            }
        }
    }

    #[export]
    fn get_frames_behind_host(&mut self, _owner: &Node) -> u32 {
        match &mut self.sess {
            Some(s) => return s.frames_behind_host(),
            None => {
                godot_error!("No session was made");
                return 0;
            }
        }
    }

    #[export]
    fn set_catchup_speed(&mut self, _owner: &Node, desired_catchup_speed: u32) {
        match &mut self.sess {
            Some(s) => match s.set_catchup_speed(desired_catchup_speed) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("No session was made"),
        }
    }

    #[export]
    fn set_max_frames_behind(&mut self, _owner: &Node, desired_value: u32) {
        match &mut self.sess {
            Some(s) => match s.set_max_frames_behind(desired_value) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("No session was made"),
        }
    }

    #[export]
    fn poll_remote_clients(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => s.poll_remote_clients(),
            None => godot_error!("No session made."),
        }
    }

    #[export]
    fn set_fps(&mut self, _owner: &Node, fps: u32) {
        match &mut self.sess {
            Some(s) => match s.set_fps(fps) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("No session made."),
        }
    }

    #[export]
    fn print_network_stats(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.network_stats() {
                Ok(n) => godot_print!("send_queue_len: {0}; ping: {1}; kbps_sent: {2}; local_frames_behind: {3}; remote_frames_behind: {4};", n.send_queue_len, n.ping, n.kbps_sent, n.local_frames_behind, n.remote_frames_behind),
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("No session made."),
        }
    }
    //NON-EXPORTED FUNCTIONS
    fn handle_requests(&mut self, requests: Vec<GGRSRequest>) {
        for item in requests {
            match item {
                GGRSRequest::AdvanceFrame { inputs } => self.ggrs_request_advance_fame(inputs),
                GGRSRequest::LoadGameState { cell } => self.ggrs_request_load_game_state(cell),
                GGRSRequest::SaveGameState { cell, frame } => {
                    self.ggrs_request_save_game_state(cell, frame);
                }
            }
        }
    }

    ////GGRSRequest handlers
    fn ggrs_request_advance_fame(&self, inputs: Vec<ggrs::GameInput>) {
        //Parse parameter inputs in a way that godot can handle then call the callback method
        match self.callback_node {
            Some(s) => {
                let node = unsafe { s.assume_safe() };
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
                unsafe { node.call("ggrs_advance_frame", &[godot_array.to_variant()]) };
            }
            None => {
                godot_error!("No callback node was specified.");
            }
        }
    }

    fn ggrs_request_load_game_state(&self, cell: GameStateCell) {
        //Unpack the cell and have over it's values to godot so it can handle it.
        match self.callback_node {
            Some(s) => {
                let node = unsafe { s.assume_safe() };
                let game_state = cell.load();
                let frame = game_state.frame.to_variant();
                let buffer =
                    ByteArray::from_vec(game_state.buffer.unwrap_or_default()).to_variant();
                let checksum = game_state.checksum.to_variant();
                unsafe { node.call("ggrs_load_game_state", &[frame, buffer, checksum]) };
            }
            None => {
                godot_error!("No callback node was specified.");
            }
        }
    }

    fn ggrs_request_save_game_state(&mut self, cell: GameStateCell, frame: Frame) {
        //Store current cell for later use
        match self.callback_node {
            Some(s) => {
                let node = unsafe { s.assume_safe() };
                let state: Variant =
                    unsafe { node.call("ggrs_save_game_state", &[frame.to_variant()]) };
                let state_bytes = ByteArray::from_variant(&state).unwrap_or_default();
                let mut state_bytes_vec = Vec::new();
                for i in 0..state_bytes.len() {
                    state_bytes_vec.push(state_bytes.get(i));
                }
                let result = GameState::new(frame, Some(state_bytes_vec), None);
                cell.save(result);
            }
            None => {
                godot_error!("No callback node was specified.");
            }
        }
    }
}
