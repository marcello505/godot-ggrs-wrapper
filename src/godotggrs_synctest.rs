use gdnative::prelude::*;
use ggrs::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRSSyncTest {
    sess: Option<SyncTestSession>,
    callback_node: Option<Ref<Node>>,
}

impl GodotGGRSSyncTest {
    fn new(_owner: &Node) -> Self {
        GodotGGRSSyncTest {
            sess: None,
            callback_node: None,
        }
    }
}

#[methods]
impl GodotGGRSSyncTest {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("GodotGGRSP2PSession _ready() called.");
    }
}
