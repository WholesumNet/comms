use futures::{future::Either};
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade},
    gossipsub, identity, mdns, noise, request_response,
    swarm::{Swarm, NetworkBehaviour, StreamProtocol, SwarmBuilder},
    tcp, yamux, PeerId, Transport,
};
use libp2p_quic as quic;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

use crate::notice;


#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    pub req_resp: request_response::cbor::Behaviour<notice::Request, notice::Response>,
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::async_io::Behaviour,
}

pub fn setup_local_swarm() -> Swarm<MyBehaviour> {
    // get a random peer_id
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(id_keys.public());
    println!("PeerId: {local_peer_id}");
    // setup an encrypted dns-enabled transport over yamux
    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&id_keys).expect("signing libp2p static keypair"))
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(30))
        .boxed();
    let quic_transport = quic::async_std::Transport::new(quic::Config::new(&id_keys));
    let transport = OrTransport::new(quic_transport, tcp_transport)
        .map(|either_output, _| match either_output {
            Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
        })
        .boxed();

    // to content-address message, take the hash of message and use it as an id
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // set a custom Gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // aid debugging by not cluttering log space
        .validation_mode(gossipsub::ValidationMode::Strict) // enforce message signing
        .message_id_fn(message_id_fn) // content-address messages
        .build()
        .expect("Invalid gossipsub config.");

    // build a Gossipsub network behaviour
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    )
    .expect("Invalid behaviour configuration.");

    // subscribe to our topic
    const TOPIC_OF_INTEREST: &str = "<-- Compute Bazaar -->";
    println!("topic of interest: `{TOPIC_OF_INTEREST}`");
    // @ use topic_hash config for auto hash(topic)
    let topic = gossipsub::IdentTopic::new(TOPIC_OF_INTEREST);
    let _ = gossipsub.subscribe(&topic);

    // create a swarm to manage events and peers
    let swarm = {
        let mdns = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)
            .expect("Failed to setup mdns behaviour.");
        let req_resp = request_response::cbor::Behaviour::<notice::Request, notice::Response>::new(
            [(
                StreamProtocol::new("/p2pcompute"),
                request_response::ProtocolSupport::Full,
            )],
            request_response::Config::default(),
        );
        let behaviour = MyBehaviour {
            req_resp: req_resp,
            gossipsub: gossipsub,
            mdns: mdns,
        };
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
    };
    swarm
}