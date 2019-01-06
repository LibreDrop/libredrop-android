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
use jni::objects::{JClass, JObject, JValue};
use libredrop_net::{discover_peers, PeerInfo};
use log::Level;
use safe_crypto::gen_encrypt_keypair;
use std::cell::RefCell;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::ops::Deref;
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
pub extern "C" fn Java_io_libredrop_network_Network_startDiscovery(env: JNIEnv, object: JObject) {
    trace!("Start discovery");

    let java_context = JavaContext::new(env, object);

    start_discovery(java_context);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_stopDiscovery(_env: JNIEnv, _object: JObject) {
    trace!("Stop discovery");
}

fn start_discovery(java_context: JavaContext) -> io::Result<()> {
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
                java_context.send_peer_info_to_java(peer);
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

    PEERS.with(|p: &RefCell<Vec<PeerInfo>>| {
        let index = p.borrow().len();
        let mut peers = p.borrow_mut();
        peers.push(peer_info.clone());
    });
}

struct JavaContext<'a> {
    env: JNIEnv<'a>,
    network_object: JObject<'a>,
}

const JAVA_CLASS_PEER_INFO: &str = "io/libredrop/network/PeerInfo";
const JAVA_CLASS_NETWORK: &str = "io/libredrop/network/Network";

impl<'a> JavaContext<'a> {
    fn new(env: JNIEnv<'a>, network_object: JObject<'a>) -> Self {
        Self { env, network_object }
    }

    fn create_java_peer_info(&self, peer_info: &PeerInfo) -> JObject {
        let name = self.env.new_string(peer_info.pub_key.to_string())
            .expect("Failed to create name string");

        let ip = self.env.new_string(peer_info.addr.ip().to_string())
            .expect("Failed to create ip string");

        let args: [JValue; 2] = [JValue::Object(*name.deref()), JValue::Object(*ip.deref())];

        trace!("Looking for class {}", JAVA_CLASS_PEER_INFO);

        let class = self.env.find_class(JAVA_CLASS_PEER_INFO)
            .expect(&format!("Failed to find {} class", JAVA_CLASS_PEER_INFO));

        self.env.new_object(class, "(Ljava/lang/String;Ljava/lang/String;)V", &args)
            .expect(&format!("Failed create new object"))
    }

    fn send_peer_info_to_java(&self, peer_info: &PeerInfo) {
        let java_peer_info = self.create_java_peer_info(peer_info);
        let args: [JValue; 1] = [JValue::Object(java_peer_info)];

        self.env.call_method(self.network_object, "onNewConnectionFound", "(Lio/libredrop/network/PeerInfo;)V", &args);
    }
}
