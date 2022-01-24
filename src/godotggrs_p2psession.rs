use crate::*;
use gdnative::core_types::ToVariant;
use ggrs::{Frame, GGRSEvent, P2PSession, PlayerHandle, PlayerType, SessionState};
use std::option::*;

/// A Godot implementation of [`P2PSession`]
#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotGGRSP2PSession {
    sess: Option<P2PSession>,
    callback_node: Option<Ref<Node>>,
    next_handle: usize,
}

impl GodotGGRSP2PSession {
    fn new(_owner: &Node) -> Self {
        GodotGGRSP2PSession {
            sess: None,
            callback_node: None,
            next_handle: 0,
        }
    }
}

#[methods]
impl GodotGGRSP2PSession {
    //EXPORTED FUNCTIONS
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("GodotGGRSP2PSession _ready() called.");
    }

    /// Creates a [P2PSession],
    /// call this when you want to start setting up a P2P Session takes the local port, total number of players and max prediction frames as parameters.
    /// # Notes
    /// - Max prediction frames is the maximum number of frames GGRS will roll back. Every gamestate older than this is guaranteed to be correct if the players did not desync.
    /// - This value used to default to `8 frames`, but this has been made adjustable with `GGRS 0.7.0`
    #[export]
    pub fn create_session(
        &mut self,
        _owner: &Node,
        local_port: u16,
        num_players: u32,
        max_pred: usize,
    ) {
        let input_size: usize = std::mem::size_of::<u32>();
        match P2PSession::new(num_players, input_size, max_pred, local_port) {
            Ok(s) => self.sess = Some(s),
            Err(e) => godot_error!("{}", e),
        }
    }

    /// Adds a local player to the session and return the handle.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn add_local_player(&mut self, _owner: &Node) -> PlayerHandle {
        self.add_player(PlayerType::Local)
    }

    /// Adds a remote player to the session and returns the handle.
    /// # Example
    /// The following example shows how to format an address string, starting with the IP and ending with the port.
    /// ```
    /// p2p.add_remote_player("127.0.0.1:7070")
    /// ```
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    /// - Will panic if the address string could not be converted to an [std::net::SocketAddr]
    #[export]
    pub fn add_remote_player(&mut self, _owner: &Node, address: String) -> PlayerHandle {
        let remote_addr: std::net::SocketAddr = address.parse().unwrap();
        self.add_player(PlayerType::Remote(remote_addr))
    }

    /// Adds a spectator to the session and returns the handle
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    /// - Will panic if the address string could not be converted to an [std::net::SocketAddr]
    #[export]
    pub fn add_spectator(&mut self, _owner: &Node, address: String) -> PlayerHandle {
        let remote_addr: std::net::SocketAddr = address.parse().unwrap();
        self.add_player(PlayerType::Spectator(remote_addr))
    }

    /// Starts the [P2PSession]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn start_session(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => match s.start_session() {
                Ok(_) => godot_print!("Started GodotGGRS session"),
                Err(e) => {
                    godot_error!("{}", e);
                }
            },
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE)
            }
        }
    }

    /// Returns true if connection has been established with remote players and is ready to start taking inputs via [Self::advance_frame()]
    #[export]
    pub fn is_running(&mut self, _owner: &Node) -> bool {
        match &mut self.sess {
            Some(s) => s.current_state() == SessionState::Running,
            None => false,
        }
    }

    /// Returns the current sate of the session as a String. Take a look at [SessionState] for all possible states.
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

    /// This function will advance the frame using the inputs given as a parameter (currently an int in Godot)
    /// Before using this function you have to set the callback node and make sure it has the following callback functions implemented
    /// - [CALLBACK_FUNC_SAVE_GAME_STATE]
    /// - [CALLBACK_FUNC_LOAD_GAME_STATE]
    /// - [CALLBACK_FUNC_SAVE_GAME_STATE]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    /// - Will print a [ERR_MESSAGE_NO_CALLBACK_NODE] error if a callback node has not been set
    #[export]
    pub fn advance_frame(&mut self, _owner: &Node, local_player_handle: usize, local_input: u32) {
        //Convert local_input into a byte array
        let local_input_bytes = local_input.to_be_bytes();
        let local_input_array_slice: &[u8] = &local_input_bytes[..];

        match self.callback_node {
            Some(callback_node) => match &mut self.sess {
                Some(s) => match s.advance_frame(local_player_handle, local_input_array_slice) {
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

    /// Sets [P2PSession::set_fps()]
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

    /// Sets the callback node that will be called when using [Self::advance_frame()]
    #[export]
    pub fn set_callback_node(&mut self, _owner: &Node, callback: Ref<Node>) {
        self.callback_node = Some(callback);
    }

    /// Calls [P2PSession::poll_remote_clients()]
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn poll_remote_clients(&mut self, _owner: &Node) {
        match &mut self.sess {
            Some(s) => s.poll_remote_clients(),
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Prints out network stats of specified handle
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn print_network_stats(&mut self, _owner: &Node, handle: PlayerHandle) {
        match &mut self.sess {
            Some(s) => match s.network_stats(handle) {
                Ok(n) => godot_print!("send_queue_len: {0}; ping: {1}; kbps_sent: {2}; local_frames_behind: {3}; remote_frames_behind: {4};", n.send_queue_len, n.ping, n.kbps_sent, n.local_frames_behind, n.remote_frames_behind),
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Will return network stats of specified handle as a `tuple`, which will be converted to an `Array` inside godot.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_network_stats(
        &mut self,
        _owner: &Node,
        handle: PlayerHandle,
    ) -> (usize, u64, usize, i32, i32) {
        const DEFAULT_RESPONSE: (usize, u64, usize, i32, i32) = (0, 0, 0, 0, 0);
        match &mut self.sess {
            Some(s) => match s.network_stats(handle) {
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

    /// Sets [P2PSession::set_frame_delay()] of specified handle.
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

    /// Sets [P2PSession::set_disconnect_timeout()] converting the u64 to secconds.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_disconnect_timeout(&mut self, _owner: &Node, secs: u64) {
        match &mut self.sess {
            Some(s) => s.set_disconnect_timeout(std::time::Duration::from_secs(secs)),
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Sets [P2PSession::set_disconnect_notify_delay()] converting the u64 to secconds.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_disconnect_notify_delay(&mut self, _owner: &Node, secs: u64) {
        match &mut self.sess {
            Some(s) => s.set_disconnect_notify_delay(std::time::Duration::from_secs(secs)),
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Sets [P2PSession::set_sparse_saving()].
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn set_sparse_saving(&mut self, _owner: &Node, sparse_saving: bool) {
        match &mut self.sess {
            Some(s) => match s.set_sparse_saving(sparse_saving) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
        }
    }

    /// Disconnects specified player handle.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn disconnect_player(&mut self, _owner: &Node, player_handle: PlayerHandle) {
        match &mut self.sess {
            Some(s) => match s.disconnect_player(player_handle) {
                Ok(_) => return,
                Err(e) => godot_error!("{}", e),
            },
            None => godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE),
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

    /// Calls and returns [P2PSession::frames_ahead()].
    /// Will return a 0 if no session was made.
    /// # Errors
    /// - Will print a [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_frames_ahead(&mut self, _owner: &Node) -> i32 {
        match &mut self.sess {
            Some(s) => s.frames_ahead(),
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                0
            }
        }
    }

    /// Calls and returns [P2PSession::max_prediction()].
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

    /// Calls and returns [P2PSession::current_frame()].
    /// Will return a 0 if no session was made.
    /// # Errors
    /// - Will print an [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_current_frame(&mut self, _owner: &Node) -> Frame {
        match &mut self.sess {
            Some(s) => s.current_frame(),
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                return 0;
            }
        }
    }

    /// Calls and returns [P2PSession::confirmed_frame()].
    /// Will return a 0 if no session was made.
    /// # Errors
    /// - Will print an [ERR_MESSAGE_NO_SESSION_MADE] error if a session has not been made
    #[export]
    pub fn get_confirmed_frame(&mut self, _owner: &Node) -> Frame {
        match &mut self.sess {
            Some(s) => s.confirmed_frame(),
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                return 0;
            }
        }
    }

    //NON-EXPORTED FUNCTIONS
    fn add_player(&mut self, player_type: PlayerType) -> PlayerHandle {
        match &mut self.sess {
            Some(s) => match s.add_player(player_type, self.next_handle) {
                Ok(o) => {
                    self.next_handle += 1;
                    return o;
                }
                Err(e) => {
                    godot_error!("{}", e);
                    panic!()
                }
            },
            None => {
                godot_error!("{}", ERR_MESSAGE_NO_SESSION_MADE);
                panic!()
            }
        };
    }
}
