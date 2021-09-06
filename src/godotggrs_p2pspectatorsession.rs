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
}
