use crate::*;
use ggrs::{SessionBuilder, PlayerHandle, PlayerType};

// A Godot implementation of [`SessionBuilder`]
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRSSessionBuilder {
    sess_builder: SessionBuilder,
    callback_node: Option<Ref<Node>>,
}

impl GodotGGRSSessionBuilder{
    fn new(_owner: $Node) -> Self{
        GodotGGRSSessionBuilder{
            sess_builder: SessionBuilder::new(),
            callback_node: None,
        }
    }
}

#[methods]
impl GodotGGRSSessionBuilder{
    //Exported Functions
    #[export]
    pub fn add_local_player(&self, _owner: $Node, player_handle: PlayerHandle){
        match self.sess_builder.add_player(PlayerType::Local, player_handle){
            Ok(_) => {},
            Err(e)=>{
                godot_error!("{}", e);
            }
        }
    }
}