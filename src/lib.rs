use std::option::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRS{
    sess : Option<ggrs::P2PSession>,
    callback_nodepath: String
}


impl GodotGGRS {
    fn new(_owner: &Node) -> Self {
        GodotGGRS{
            sess: None,
            callback_nodepath : "../.".to_string()
        }
    }
}

#[methods]
impl GodotGGRS{
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.");
    }

    #[export]
    fn create_session(&mut self, _owner: &Node, local_port: u16, num_players: u32){
        let input_size : usize = std::mem::size_of::<u32>();
        match ggrs::start_p2p_session(num_players, input_size, local_port){
            Ok(s) => self.sess = Some(s),
            Err(e) => godot_print!("{:?}", e),
        }
    }

    #[export]
    fn receive_callback_node(&mut self, _owner: &Node, nodepath : String){
        self.callback_nodepath = nodepath;
    }

}



fn init(handle: InitHandle) {
    handle.add_class::<GodotGGRS>();
}

godot_init!(init);