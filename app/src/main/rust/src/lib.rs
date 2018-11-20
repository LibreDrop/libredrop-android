extern crate android_logger;
extern crate jni;
extern crate libredrop_net;
#[macro_use]
extern crate log;

use android_logger::Filter;
use jni::JNIEnv;
use jni::objects::JClass;
use libredrop_net::discover_peers;
use log::Level;
use std::sync::Once;

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
pub extern "C" fn Java_io_libredrop_android_MainActivity_startDiscovery(env: JNIEnv,
                                                                        _class: JClass) {
    init();
}
