use gdnative::core_types::ToVariant;
use gdnative::prelude::*;
use ggrs::*;
use std::option::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRS {
    sess: Option<P2PSession>,
    callback_node: Option<Ref<Node>>,
    next_handle: usize,
    stored_cell: GameStateCell,
}

impl GodotGGRS {
    fn new(_owner: &Node) -> Self {
        GodotGGRS {
            sess: None,
            callback_node: None,
            next_handle: 0,
            stored_cell: GameStateCell::default(),
        }
    }
}

#[methods]
impl GodotGGRS {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("GodotGGRS _ready() called.");
    }

    #[export]
    fn create_session(&mut self, _owner: &Node, local_port: u16, num_players: u32) {
        let input_size: usize = std::mem::size_of::<u32>();
        match start_p2p_session(num_players, input_size, local_port) {
            Ok(s) => self.sess = Some(s),
            Err(e) => godot_print!("{:?}", e),
        }
    }

    #[export]
    fn add_local_player(&mut self, _owner: &Node) -> usize {
        self.add_player(PlayerType::Local)
    }

    #[export]
    fn add_remote_player(&mut self, _owner: &Node, address: String) -> usize {
        let remote_addr: std::net::SocketAddr = address.parse().unwrap();
        self.add_player(PlayerType::Remote(remote_addr))
    }

    #[export]
    fn add_spectator(&mut self, _owner: &Node, address: String) -> usize {
        let remote_addr: std::net::SocketAddr = address.parse().unwrap();
        self.add_player(PlayerType::Spectator(remote_addr))
    }

    #[export]
    fn start_session(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.start_session() {
                Ok(_) => godot_print!("Started GodotGGRS session"),
                Err(e) => {
                    godot_print!("{}", e);
                    panic!()
                }
            },
            None => {}
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
    fn advance_frame(&mut self, _owner: &Node, local_player_handle: usize, local_input: ByteArray) {
        if self.callback_node.is_none() {
            godot_print!("Can't advance frame, no callback_node was set");
            panic!();
        }
        if self.sess.is_none() {
            godot_print!("Can't advance frame, no session was created");
            panic!();
        }
        let mut local_input_array: Vec<u8> = Vec::new();
        //Convert local_input into a Rust parsable array
        for i in 0..local_input.len() {
            local_input_array.push(local_input.get(i));
        }
        let local_input_array_slice: &[u8] = &local_input_array[..];

        match &mut self.sess {
            Some(s) => match s.advance_frame(local_player_handle, local_input_array_slice) {
                Ok(requests) => {
                    self.handle_requests(requests);
                }
                Err(e) => {
                    godot_print!("{}", e);
                    panic!();
                }
            },
            None => {
                godot_print!("No session was made");
                panic!();
            }
        }
    }

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

    fn ggrs_request_advance_fame(&self, inputs: Vec<ggrs::GameInput>) {
        //Parse parameter inputs in a way that godot can handle then call the callback method
        match self.callback_node {
            Some(s) => {
                let node = unsafe { s.assume_safe() };
                let mut godot_array = Vec::new();
                for i in inputs {
                    let result =
                        (i.frame, i.size, ByteArray::from_slice(&i.buffer[..])).to_variant();
                    godot_array.push(result);
                }
                unsafe { node.call("ggrs_advance_frame", &godot_array[..]) };
            }
            None => {
                godot_print!("No callback node was specified");
                panic!();
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
                let buffer = game_state.buffer.unwrap_or_default().to_variant();
                let checksum = game_state.checksum.to_variant();
                unsafe { node.call("ggrs_load_game_state", &[frame, buffer, checksum]) };
            }
            None => {
                godot_print!("No callback node was specified.");
                panic!();
            }
        }
    }

    fn ggrs_request_save_game_state(&mut self, cell: GameStateCell, frame: Frame) {
        //Store current cell for later use
        self.stored_cell = cell;
        match self.callback_node {
            Some(s) => {
                let node = unsafe { s.assume_safe() };
                unsafe { node.call("ggrs_save_game_state", &[frame.to_variant()]) };
            }
            None => {
                godot_print!("No callback node was specified.");
                panic!();
            }
        }
    }

    #[export]
    fn save_game_state(&self, _owner: &Node, frame: Frame, buffer: ByteArray, checksum: u64) {
        //This should be called by the callback node when it's ready to save the state
        let mut buffer_vec: Vec<u8> = Vec::new();
        for i in 0..buffer.len() {
            buffer_vec.push(buffer.get(i));
        }
        let result = GameState {
            frame: frame,
            buffer: Some(buffer_vec),
            checksum: checksum,
        };
        self.stored_cell.save(result);
    }

    fn add_player(&mut self, player_type: PlayerType) -> usize {
        match &mut self.sess {
            Some(s) => match s.add_player(player_type, self.next_handle) {
                Ok(o) => {
                    self.next_handle += 1;
                    return o;
                }
                Err(e) => {
                    godot_print!("{}", e);
                    panic!()
                }
            },
            None => {
                godot_print!("No session was made.");
                panic!()
            }
        };
    }

    #[export]
    fn receive_callback_node(&mut self, _owner: &Node, callback: Ref<Node>) {
        self.callback_node = Some(callback);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<GodotGGRS>();
}

godot_init!(init);
