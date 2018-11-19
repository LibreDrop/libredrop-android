extern crate jni;
extern crate libredrop_net;

use jni::JNIEnv;
use jni::objects::JClass;
use libredrop_net::discover_peers;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_android_MainActivity_startDiscovery(env: JNIEnv,
                                                                        _class: JClass) {}
