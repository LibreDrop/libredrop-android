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

use android_logger::Filter;
use futures::Stream;
use get_if_addrs::{get_if_addrs, IfAddr};
use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JObject;
use libredrop_net::{discover_peers, PeerInfo};
use log::Level;
use safe_crypto::gen_encrypt_keypair;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Once;
use std::vec::Vec;
use tokio::runtime::current_thread::Runtime;

static START: Once = Once::new();

fn init() {
    START.call_once(|| {
        android_logger::init_once(
            Filter::default()
                .with_min_level(Level::Trace),
            Some("native"),
        );

        trace!("Initialization complete");
    });
}

thread_local! {
    pub static PEERS: RefCell<Vec<PeerInfo>> = RefCell::new(Vec::new());
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_init(_env: JNIEnv, _class: JClass) {
    init();
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_startDiscovery(_env: JNIEnv, _object: JObject) {
    trace!("Start discovery");

    start_discovery();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_stopDiscovery(_env: JNIEnv, _object: JObject) {
    trace!("Stop discovery");
}

fn start_discovery() -> io::Result<()> {
    let mut evloop = unwrap!(Runtime::new());

    trace!("Looking for peers on LAN on port 6000");
    let mut addrs = our_addrs(1234)?;
    trace!("Our addr: {:?}", addrs);
    let (our_pk, our_sk) = gen_encrypt_keypair();
    let find_peers = discover_peers(6000, addrs, &our_pk, &our_sk);

    if let Err(ref e) = find_peers {
        trace!("discovery_peers() failed with {:?}", e);
    }

    let find_peers = unwrap!(find_peers)
        .map_err(|e| error!("Peer discovery failed: {:?}", e))
        .for_each(|peers: Vec<PeerInfo>| {
            peers.iter().for_each(|peer| {
                add_peer(peer);
            });
            Ok(())
        });
    unwrap!(evloop.block_on(find_peers));
    Ok(())
}

fn our_addrs(with_port: u16) -> io::Result<Vec<SocketAddr>> {
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

fn add_peer(peer_info: &PeerInfo) {
    trace!("Peer is listening on: {:?}", peer_info);

//    PEERS.with(|p: RefCell<Vec<PeerInfo>>| {
//        let index = p.borrow().len();
//        let &peers: Vec<PeerInfo> = p.borrow_mut().as_mut();
//        peers.append(peer_info);
//    });
}
