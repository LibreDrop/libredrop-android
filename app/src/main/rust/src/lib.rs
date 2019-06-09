extern crate android_logger;
extern crate futures;
extern crate get_if_addrs;
extern crate jni;
extern crate libredrop_net;
#[macro_use]
extern crate log;
extern crate tokio;
#[macro_use]
extern crate unwrap;

use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::option::Option;
use std::sync::Once;
use std::vec::Vec;

use android_logger::Config;
use futures::{future, Future, Sink, Stream};
use get_if_addrs::{get_if_addrs, IfAddr};
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use libredrop_net::{Connection, Message, Peer, PeerEvent, PeerInfo};
use log::Level;
use tokio::runtime::current_thread::Runtime;

use java_context::JavaContext;

mod java_context;

static START: Once = Once::new();

fn init() {
    START.call_once(|| {
        android_logger::init_once(
            Config::default().with_min_level(Level::Trace)
        );

        trace!("Initialization complete");
    });
}

thread_local! {
    pub static APP: RefCell<Option<App<'static>>> = RefCell::new(Option::None);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_init(_env: JNIEnv, _class: JClass) {
    init();
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_startDiscovery(env: JNIEnv<'static>, object: JObject<'static>) {
    trace!("Start discovery");

    APP.with(|cell| {
        let java_context = JavaContext::new(env, object);
        let mut app = App::new(java_context);
        app.start_discovery();
        cell.borrow_mut().replace(app);
    });
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_stopDiscovery(_env: JNIEnv, _object: JObject) {
    trace!("Stop discovery");
}

pub struct App<'a> {
    evloop: Runtime,
    peers: RefCell<Vec<PeerInfo>>,
    java_context: JavaContext<'a>,
}

impl<'a> App<'a> {
    fn new(java_context: JavaContext<'a>) -> Self {
        let evloop = unwrap!(Runtime::new());
        let peers = RefCell::new(Vec::new());

        Self { evloop, peers, java_context }
    }

    fn start_discovery(&mut self) -> io::Result<()> {
        trace!("Looking for peers on LAN on port 6000");
        let addrs = App::our_addrs(1234)?;
        trace!("Our addr: {:?}", addrs);
        let (mut peer, peer_events_rx) = Peer::new(6000);

        let handle_peer_events = peer_events_rx
            .for_each(|event: PeerEvent| {
                match event {
                    PeerEvent::DiscoveredPeers(peers) => {
                        peers.iter().for_each(|peer| {
                            trace!("New peer: {:?}", peer);
//                            let index = self.add_peer(peer);
//                            self.java_context.send_peer_info_to_java(peer, index);
                        });
                    }
                    PeerEvent::NewConnection(conn) => {
                        trace!("New connection: {:?}", conn);
                    }
                }
                Ok(())
            })
            .map_err(|_| { () });

        self.evloop.spawn(handle_peer_events);
        unwrap!(peer.start(&mut self.evloop));

        Ok(())
    }

    fn our_addrs(with_port: u16) -> io::Result<HashSet<SocketAddr>> {
        let interfaces = get_if_addrs()?;
        let addrs = interfaces
            .iter()
            .filter_map(|interface| match interface.addr {
                IfAddr::V4(ref ifv4_addr) => Some(ifv4_addr.ip),
                IfAddr::V6(_) => None,
            }).filter(|ip| !ip.is_loopback())
            .map(|ip| SocketAddr::V4(SocketAddrV4::new(ip, with_port)))
            .collect();
        Ok(addrs)
    }

    fn add_peer(self, peer_info: &PeerInfo) -> usize {
        trace!("Peer is listening on: {:?}", peer_info);

        let mut peers = self.peers.borrow_mut();
        peers.push(peer_info.clone());
        peers.len() - 1
    }
}
