use std::{
    error::Error,
    time::Duration,
    hash::{
        Hash, Hasher
    },
    collections::hash_map::DefaultHasher
};

use libp2p::{
    gossipsub, identity, identify, kad, mdns, noise, request_response,
    swarm::{Swarm, NetworkBehaviour, StreamProtocol},
    SwarmBuilder,
    tcp, yamux, PeerId,
};

use crate::notice;

// prepare gossipsub behaviour
pub fn prepare_gossipsub_behaviour(
    keypair: &identity::Keypair,
    topic: &str
)-> Result<gossipsub::Behaviour, Box<dyn Error + Send + Sync>> {
    // content-address messages
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };
    // set a custom Gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // aid debugging by not cluttering log space
        .validation_mode(gossipsub::ValidationMode::Strict) // enforce message signing
        .message_id_fn(message_id_fn) 
        .build()?;
    let mut gossipsub_behaviour = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(keypair.clone()),
        gossipsub_config
    )?;
    let topic = gossipsub::IdentTopic::new(topic);
    let _ = gossipsub_behaviour.subscribe(&topic);
    Ok(gossipsub_behaviour)
}

// prepare request-response behaviour
pub fn prepare_request_response_behaviour()
-> request_response::cbor::Behaviour<notice::Request, notice::Response> 
{
    request_response::cbor::Behaviour::<notice::Request, notice::Response>::new(
        [(
            StreamProtocol::new("/wholesum/req_resp/1.0"),
            request_response::ProtocolSupport::Full,
        )],
        request_response::Config::default(),
    )
}

// prepare identify behaviour
pub fn prepare_identify_behaviour(
    public_key: &identity::PublicKey
)-> identify::Behaviour {
    identify::Behaviour::new(
        identify::Config::new(
            String::from("/wholesum/identify/1.0"),
            public_key.clone()
        )
    )
}

#[derive(NetworkBehaviour)]
pub struct LocalBehaviour {
    pub req_resp: request_response::cbor::Behaviour<notice::Request, notice::Response>,
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::async_io::Behaviour,
}

pub fn setup_local_swarm(
    keypair: &identity::Keypair,
    topic: &str,
)-> Result<Swarm<LocalBehaviour>, Box<dyn Error>> {
    let local_keypair = keypair.clone();
    let swarm = SwarmBuilder::with_existing_identity(local_keypair.clone())
        .with_async_std()
        // .with_tcp(
        //     tcp::Config::default(),
        //     noise::Config::new,
        //     yamux::Config::default
        // )?
        .with_quic()     
        .with_behaviour(|key| {
            // setup identify
            // let identify = {
            //     identify::Behaviour::new(
            //         identify::Config::new(
            //             String::from("/wholesum/identify/1.0"),
            //             key.public()
            //         )
            //     )
            // };

            // setup mdns
            let local_peer_id = identity::PeerId::from_public_key(&local_keypair.public());
            let mdns_behaviour = mdns::async_io::Behaviour::new(mdns::Config::default(), local_peer_id)?;
            Ok(LocalBehaviour {
                mdns: mdns_behaviour,
                gossipsub: prepare_gossipsub_behaviour(&key, topic)?,
                req_resp: prepare_request_response_behaviour(),
            })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}



#[derive(NetworkBehaviour)]
pub struct GlobalBehaviour {
    pub identify: identify::Behaviour,
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub gossipsub: gossipsub::Behaviour,
    // pub req_resp: request_response::cbor::Behaviour<notice::Request, notice::Response>,
}

// setup a global swram instance
async fn setup_global_swarm(
    keypair: &identity::Keypair
)-> Result<Swarm<GlobalBehaviour>, Box<dyn Error>> {
    let local_keypair = keypair.clone();
    let swarm = libp2p::SwarmBuilder::with_existing_identity(local_keypair)
        .with_async_std()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default
        )?
        .with_quic()
        .with_dns().await?        
        .with_behaviour(|key| {
            // setup identify
            let identify = {
                identify::Behaviour::new(
                    identify::Config::new(
                        String::from("/wholesum/identify/1.0"),
                        key.public()
                    )
                )
            };

            // setup gossipsub
            let gossipsub_behaviour = {
                // content-address messages
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };
                // set a custom Gossipsub configuration
                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10)) // aid debugging by not cluttering log space
                    .validation_mode(gossipsub::ValidationMode::Strict) // enforce message signing
                    .message_id_fn(message_id_fn) 
                    .build()?;
                gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config
                )?
            };

            // setup kademlia
            let kademlia = {
                let local_peer_id = PeerId::from(keypair.public());
                let mut cfg = kad::Config::default();
                cfg.set_query_timeout(Duration::from_secs(5 * 60));
                cfg.set_protocol_names(
                    vec![
                        StreamProtocol::new("/wholesum/kad/1.0")                        
                    ]
                );
                let store = kad::store::MemoryStore::new(local_peer_id);
                kad::Behaviour::with_config(local_peer_id, store, cfg)
            };

            Ok(GlobalBehaviour {
                identify: identify,
                kademlia: kademlia,
                gossipsub: gossipsub_behaviour,
                // req_resp: req_resp,
            })
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();
    Ok(swarm)
}
