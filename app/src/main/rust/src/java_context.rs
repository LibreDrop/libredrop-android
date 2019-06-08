extern crate jni;
extern crate libredrop_net;

use jni::errors::Result as JniResult;
use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use libredrop_net::{PeerInfo};

pub struct JavaContext<'a> {
    env: JNIEnv<'a>,
    network_object: JObject<'a>,
}

const JAVA_CLASS_PEER_INFO: &str = "io/libredrop/network/PeerInfo";
const JAVA_CLASS_NETWORK: &str = "io/libredrop/network/Network";

impl<'a> JavaContext<'a> {
    pub fn new(env: JNIEnv<'a>, network_object: JObject<'a>) -> Self {
        Self { env, network_object }
    }

    pub fn create_java_peer_info(&self, peer_info: &PeerInfo, index: usize) -> JniResult<JObject> {
        trace!("Looking for class {}", JAVA_CLASS_PEER_INFO);
        let class = self.env.find_class(JAVA_CLASS_PEER_INFO)?;

        let name = self.env.new_string(peer_info.pub_key.to_string())?;
        let ip = self.env.new_string(peer_info.addr.ip().to_string())?;
        let args = [JValue::Int(index as i32), JValue::Object(*name), JValue::Object(*ip)];

        self.env.new_object(class, "(ILjava/lang/String;Ljava/lang/String;)V", &args)
    }

    pub fn send_peer_info_to_java(&self, peer_info: &PeerInfo, index: usize) -> JniResult<JValue> {
        let java_peer_info = self.create_java_peer_info(peer_info, index)?;
        let args = [JValue::Object(java_peer_info)];

        trace!("Sending PeerInfo to Java {:?}", java_peer_info);

        self.env.call_method(self.network_object, "onNewConnectionFound", "(Lio/libredrop/network/PeerInfo;)V", &args)
    }
}
