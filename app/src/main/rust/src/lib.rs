extern crate android_logger;
extern crate futures;
extern crate get_if_addrs;
extern crate jni;
extern crate libredrop_net;
#[macro_use]
extern crate log;
extern crate safe_crypto;
extern crate tokio;
#[macro_use]
extern crate unwrap;

use std::cell::RefCell;
use std::collections::HashSet;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Once;
use std::vec::Vec;

use android_logger::Config;
use futures::Stream;
use get_if_addrs::{get_if_addrs, IfAddr};
use jni::JNIEnv;
use jni::objects::{JClass, JObject};
use libredrop_net::{discover_peers, PeerInfo};
use log::Level;
use safe_crypto::gen_encrypt_keypair;
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
    pub static APP: App = App::new();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_init(_env: JNIEnv, _class: JClass) {
    init();
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_startDiscovery(env: JNIEnv, object: JObject) {
    trace!("Start discovery");

    APP.with(|app| {
        let java_context = JavaContext::new(env, object);
        app.start_discovery(java_context);
    });
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_stopDiscovery(_env: JNIEnv, _object: JObject) {
    trace!("Stop discovery");
}

struct App {
    evloop: Runtime,
    peers: RefCell<Vec<PeerInfo>>,
}

impl App {
    fn new() -> Self {
        let evloop = unwrap!(Runtime::new());
        let peers = RefCell::new(Vec::new());

        Self { evloop, peers }
    }

    fn start_discovery(self, java_context: JavaContext) -> io::Result<()> {
        trace!("Looking for peers on LAN on port 6000");
        let addrs = App::our_addrs(1234)?;
        trace!("Our addr: {:?}", addrs);
        let (our_pk, our_sk) = gen_encrypt_keypair();
        let find_peers = discover_peers(6000, addrs, &our_pk, &our_sk);

        if let Err(ref e) = find_peers {
            trace!("discovery_peers() failed with {:?}", e);
        }

        let find_peers = unwrap!(find_peers)
            .map_err(|e| error!("Peer discovery failed: {:?}", e))
            .for_each(|peers: HashSet<PeerInfo>| {
                peers.iter().for_each(|peer| {
                    let index = self.add_peer(peer);
                    java_context.send_peer_info_to_java(peer, index);
                });
                Ok(())
            });

        self.evloop.spawn(find_peers);

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
