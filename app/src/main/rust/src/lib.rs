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

use android_logger::Filter;
use futures::Stream;
use get_if_addrs::{get_if_addrs, IfAddr};
use jni::JNIEnv;
use jni::objects::JClass;
use jni::objects::JObject;
use libredrop_net::discover_peers;
use log::Level;
use std::io;
use std::net::{SocketAddr, SocketAddrV4};
use std::sync::Once;
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

    info!("Looking for peers on LAN on port 6000");
    let addrs = our_addrs(1234)?;
    let find_peers = unwrap!(discover_peers(6000, addrs))
        .map_err(|e| error!("Peer discovery failed: {:?}", e))
        .for_each(|addrs| {
            println!("Peer is listening on: {:?}", addrs);
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
