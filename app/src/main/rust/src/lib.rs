extern crate android_logger;
extern crate core;
extern crate future_utils;
extern crate futures;
extern crate jni;
extern crate libredrop_net;
#[macro_use]
extern crate log;
extern crate tokio;
#[macro_use]
extern crate unwrap;
extern crate void;

use std::cell::RefCell;
use std::fs::copy;
use std::option::Option;
use std::sync::Once;
use std::vec::Vec;

use android_logger::Config;
use future_utils::{BoxFuture, FutureExt, mpsc};
use future_utils::mpsc::{UnboundedReceiver, UnboundedSender};
use futures::{future, Future, Stream};
use jni::JNIEnv;
use jni::objects::{JClass, JObject, JString, JValue};
use libredrop_net::{Peer, PeerEvent, PeerInfo};
use log::Level;
use tokio::runtime::current_thread::Runtime;
use void::Void;

use ::Event::{FromPeer, SendMessage};
use java_context::JavaContext;

mod java_context;

#[derive(Debug)]
pub enum Event {
    FromPeer(PeerEvent),
    SendMessage(u32, String),
}

static START: Once = Once::new();

fn init() {
    START.call_once(|| {
        android_logger::init_once(
            Config::default().with_min_level(Level::Trace)
        );

        trace!("Initialization complete");
    });
}

//TODO: replace with mutex
static mut MAIN_SENDER: Option<UnboundedSender<Event>> = Option::None;

thread_local! {
    //TODO: Replace with mutex
    pub static QUIT: RefCell<Option<UnboundedSender<()>>> = RefCell::new(Option::None);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_init(_env: JNIEnv, _class: JClass) {
    init();
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_sendMessage(env: JNIEnv, _object: JObject, peer_info: JObject, java_message: JString) {
    let index = extract_index(&env, peer_info);
    let message: String = env.get_string(java_message).unwrap().into();
    trace!("Send message to #{}: {}", index, message);

    let sender = unsafe { MAIN_SENDER.clone() };

    match sender {
        Some(tx) => {
            trace!("Main sender used for message {}", message);
            tx.unbounded_send(SendMessage(index, message));
        }
        None => { trace!("Main sender is not initialized!"); }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_startDiscovery(env: JNIEnv<'static>, object: JObject<'static>) {
    trace!("Start discovery");

    let java_context = JavaContext::new(env, object);
    let mut evloop = unwrap!(Runtime::new());
    let (app_tx, app_rx) = mpsc::unbounded::<Event>();

    let app = App::new(&mut evloop, java_context, app_tx.clone());

    unsafe {
        MAIN_SENDER = Option::Some(app_tx);
    }

    let command_future = app_rx.for_each(move |event| {
        app.handle_event(&event)
    }).map_err(|_| ());

    evloop.spawn(command_future);

    let (quit_tx, quit_rx) = mpsc::unbounded::<()>();

    QUIT.with(|a| {
        let mut option = a.borrow_mut();
        option.replace(quit_tx);
    });

    let quit_future = quit_rx.into_future().map(|_| ((), ())).map_err(|(e, _)| e);
    evloop.block_on(quit_future);

    trace!("startDiscovery finished");
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_libredrop_network_Network_stopDiscovery(_env: JNIEnv, _object: JObject) {
    trace!("Stop discovery");

    QUIT.with(|a| {
        let ref option = *a.borrow();
        match option {
            Some(tx) => { tx.unbounded_send(()); }
            None => { trace!("Quit sender is not initialized!"); }
        };
    });
}

fn extract_index(env: &JNIEnv, peer_info: JObject) -> u32 {
    let value = env.call_method(peer_info, "getId", "()I", &[]).unwrap();
    trace!("value: {:?}", value);
    match value {
        JValue::Int(int) => int as u32,
        _ => panic!("method returns non Int"),
    }
}


pub struct App<'a> {
    peers: RefCell<Vec<PeerInfo>>,
    java_context: JavaContext<'a>,
    peer: Peer,
}

impl<'a> App<'a> {
    fn new(evloop: &mut Runtime, java_context: JavaContext<'a>, app_tx: mpsc::UnboundedSender<Event>) -> Self {
        let peers = RefCell::new(Vec::new());

        let (mut peer, peer_event_rx) = Peer::new(6000);

        let handle_peer_events = peer_event_rx
            .for_each(move |event| {
                trace!("PeerEvent: {:?}", event);
                app_tx.unbounded_send(FromPeer(event));
                Ok(())
            })
            .map_err(|_| ());

        evloop.spawn(handle_peer_events);

        unwrap!(peer.start(evloop));

        Self { peers, java_context, peer }
    }

    fn handle_event(&self, event: &Event) -> BoxFuture<(), Void> {
        match event {
            FromPeer(peerEvent) => {
                match peerEvent {
                    PeerEvent::DiscoveredPeers(peers) => {
                        peers.iter().for_each(|peer| {
                            trace!("New peer: {:?}", peer);
                            let index = self.add_peer(peer);
                            self.java_context.send_peer_info_to_java(peer, index);
                        });
                    }
                    PeerEvent::NewConnection(conn) => {
                        trace!("New connection: {:?}", conn);
                    }
                }
            }

            SendMessage(index, message) => {
                trace!("Try send message to network for peer #{}", index);
            }
        }

        future::ok(()).into_boxed()
    }

    fn add_peer(&self, peer_info: &PeerInfo) -> usize {
        trace!("Peer is listening on: {:?}", peer_info);

        let mut peers = self.peers.borrow_mut();
        peers.push(peer_info.clone());
        peers.len() - 1
    }
}

