use crate::*;
use ggrs::{GGRSEvent, P2PSpectatorSession, SessionState};
use std::option::*;

/// A Godot implementation of [`P2PSpectatorSession`]
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

    /// Creates a [P2PSpectatorSession], call this when you want to start setting up a `P2PSpectatorSession`.
    /// Takes the local port and total number of players and the host address as parameters.
    #[export]
    pub fn create_session(
        &mut self,
        _owner: &Node,
        local_port: u16,
        num_players: u32,
        host_addr: String,
    ) {
        let input_size: usize = std::mem::size_of::<u32>();
        let host_addr_object: std::net::SocketAddr = host_addr.parse().unwrap();
        match P2PSpectatorSession::new(num_players, input_size, local_port, host_addr_object) {
            Ok(s) => self.sess = Some(s),
            Err(e) => godot_error!("{}", e),
        }
    }

    /// Returns true if connection has been established with remote players and is ready to start advancing frames via [Self::advance_frame()]
    #[export]
    pub fn is_running(&mut self, _owner: &Node) -> bool {
        match &mut self.sess {
            Some(s) => s.current_state() == SessionState::Running,
            None => false,
        }
    }

    /// Returns the current sate of the session as a String. See [SessionState] for all possible states.
    #[export]
    pub fn get_current_state(&mut self, _owner: &Node) -> String {
        match &mut self.sess {
            Some(s) => match s.current_state() {
                SessionState::Initializing => "Initializing".to_owned(),
                SessionState::Running => "Running".to_owned(),
                SessionState::Synchronizing => "Synchronizing".to_owned(),
            },
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                "".to_owned()
            }
        }
    }

    /// Starts the [P2PSpectatorSession]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn start_session(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.start_session() {
                Ok(_) => godot_print!("Started GodotGGRS session"),
                Err(e) => {
                    godot_error!("{}", e);
                    panic!()
                }
            },
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE)
            }
        }
    }

    /// Sets the callback node that will be called when using [Self::advance_frame()]
    #[export]
    pub fn set_callback_node(&mut self, _owner: &Node, callback: Ref<Node>) {
        self.callback_node = Some(callback);
    }

    /// This function will advance the frame using the inputs received from the host_session.
    /// Before using this function you have to set the callback node and make sure it has the following callback functions implemented
    /// - [CALLBACK_FUNC_SAVE_GAME_STATE]
    /// - [CALLBACK_FUNC_LOAD_GAME_STATE]
    /// - [CALLBACK_FUNC_SAVE_GAME_STATE]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    /// - Will print a [ERR_MESSAGE_NO_CALLBACK_NODE] error if a callback node has not been set
    #[export]
    pub fn advance_frame(&mut self, _owner: &Node) {
        match self.callback_node {
            Some(callback_node) => match &mut self.sess {
                Some(s) => match s.advance_frame() {
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

    /// Returns [P2PSpectatorSession::frames_behind_host()]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_frames_behind_host(&mut self, _owner: &Node) -> u32 {
        match &mut self.sess {
            Some(s) => return s.frames_behind_host(),
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                return 0;
            }
        }
    }

    /// Sets [P2PSpectatorSession::set_catchup_speed()]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_catchup_speed(&mut self, _owner: &Node, desired_catchup_speed: u32) {
        match &mut self.sess {
            Some(s) => match s.set_catchup_speed(desired_catchup_speed) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Sets [P2PSpectatorSession::set_max_frames_behind()]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_max_frames_behind(&mut self, _owner: &Node, desired_value: u32) {
        match &mut self.sess {
            Some(s) => match s.set_max_frames_behind(desired_value) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Calls [P2PSpectatorSession::poll_remote_clients()]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn poll_remote_clients(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => s.poll_remote_clients(),
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Sets [P2PSpectatorSession::set_fps()]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_fps(&mut self, _owner: &Node, fps: u32) {
        match &mut self.sess {
            Some(s) => match s.set_fps(fps) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Prints out network stats of host address
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn print_network_stats(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.network_stats() {
                Ok(n) => godot_print!("send_queue_len: {0}; ping: {1}; kbps_sent: {2}; local_frames_behind: {3}; remote_frames_behind: {4};", n.send_queue_len, n.ping, n.kbps_sent, n.local_frames_behind, n.remote_frames_behind),
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Will return network stats of host address as a `tuple`, which will be converted to an `Array` inside godot.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_network_stats(&mut self, _owner: &Node) -> (usize, u64, usize, i32, i32) {
        const DEFAULT_RESPONSE: (usize, u64, usize, i32, i32) = (0, 0, 0, 0, 0);
        match &mut self.sess {
            Some(s) => match s.network_stats() {
                Ok(n) => (
                    n.send_queue_len,
                    n.ping as u64,
                    n.kbps_sent,
                    n.local_frames_behind,
                    n.remote_frames_behind,
                ),
                Err(e) => {
                    godot_error!("{}", e);
                    DEFAULT_RESPONSE
                }
            },
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                DEFAULT_RESPONSE
            }
        }
    }

    /// Returns an `Array` of events which contain usefull information, while you don't have to implement everything, the one thing you should implement is the WaitRecommendation.
    /// For details regarding the events please take a loot at [GGRSEvent].
    /// # Example
    /// ```gdscript
    /// var events = ggrs.get_events()
    ///	for item in events:
    ///     match item[0]:
    ///         "WaitRecommendation":
    ///             frames_to_skip += item[1]
    ///         "NetworkInterrupted":
    ///             var handle = item[1][0]
    ///             var disconnect_timeout = item[1][1]
    ///         "NetworkResumed":
    ///             var handle = item[1]
    ///         "Disconnected":
    ///             var handle = item[1]
    ///         "Synchronized":
    ///             var handle = item[1]
    ///         "Synchronizing":
    ///             var handle = item[1][0]
    ///             var total = item[1][1]
    ///             var count = item[1][2]
    /// ```
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_events(&mut self, _owner: &Node) -> Vec<(&str, Variant)> {
        let mut result: Vec<(&str, Variant)> = Vec::new();
        match &mut self.sess {
            Some(s) => {
                for event in s.events() {
                    match event {
                        GGRSEvent::WaitRecommendation { skip_frames } => {
                            result.push(("WaitRecommendation", skip_frames.to_variant()))
                        }
                        GGRSEvent::NetworkInterrupted {
                            player_handle,
                            disconnect_timeout,
                        } => result.push((
                            "NetworkInterrupted",
                            (player_handle, disconnect_timeout as u64).to_variant(),
                        )),
                        GGRSEvent::NetworkResumed { player_handle } => {
                            result.push(("NetworkResumed", player_handle.to_variant()))
                        }
                        GGRSEvent::Disconnected { player_handle } => {
                            result.push(("Disconnected", player_handle.to_variant()))
                        }
                        GGRSEvent::Synchronized { player_handle } => {
                            result.push(("Synchronized", player_handle.to_variant()))
                        }
                        GGRSEvent::Synchronizing {
                            player_handle,
                            total,
                            count,
                        } => result
                            .push(("Synchronizing", (player_handle, total, count).to_variant())),
                    }
                }
            }
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        };
        return result;
    }
}
