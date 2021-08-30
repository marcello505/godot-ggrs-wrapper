use gdnative::prelude::*;
use ggrs::*;
use std::option::*;
use gdnative::core_types::ToVariant;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRS {
    sess: Option<P2PSession>,
    callback_node: Option<Ref<Node>>,
    next_handle: usize,
}

impl GodotGGRS {
    fn new(_owner: &Node) -> Self {
        GodotGGRS {
            sess: None,
            callback_node: None,
            next_handle: 0,
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
    fn start_session(&mut self, _owner: &Node){
        match &mut self.sess{
            Some(s) => match s.start_session(){
                Ok(_) => godot_print!("Started GodotGGRS session"),
                Err(e) => {
                    godot_print!("{}", e);
                    panic!()
                }
            }
            None => {}
        }
    }

    #[export]
    fn advance_frame(&mut self, _owner: &Node, local_player_handle: usize, local_input: [u8; 32]){
        if self.callback_node.is_none(){
            godot_print!("Can't advance frame, no callback_node was set");
            panic!();
        }
        if self.sess.is_none(){
            godot_print!("Can't advance frame, no session was created");
            panic!();
        }
        let callback = &self.callback_node.unwrap();
        let sess = &self.sess.unwrap();

        match sess.advance_frame(local_player_handle, &local_input){
            Ok(requests) => {
                for item in requests{
                    match item{
                        GGRSRequest::AdvanceFrame {inputs} => unsafe{ callback.call("ggrs_advance_frame", &[inputs.to_variant()]) },
                    }
                }
            },
            Err(e)=>{
                godot_print!("{}", e);
                panic!()
            }
        }
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
