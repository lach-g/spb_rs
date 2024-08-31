// Sparkplug-B Edge Node Steps:
// 1. Create Client struct
//    Info:
//    - URI
//    - Primary Host Application
//    Actions:
//    - Maybe set up any required data structures for Sparkplug-B?
// 2. Connect call
//    Info:
//    - NBIRTH payload of metrics going to publish
//    Actions:
//    - If Primary Host Application, check STATE
//    - Sub to spBv1.0/#
//    - Send NBIRTH
// 3. Build a message
//    - Using the prebuilt struct given to the connect call, fill in whatever values are required
// 4. Publish call
//    - Check STATE if Primary Host Application
//    - Create next Sparkplug-B encoded msg

use paho_mqtt;
use std::{env, process, time::Duration};

// > cargo run --example edgenode_sync_publish
fn main() {
    // Create a client & define connect options
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    // 1. Configure client
    let  spb_edgenode_cli = spb_rs::EdgeNode::new(host).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // 2. Connect
    if let Err(e) = spb_edgenode_cli.connect(None) {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    // 3. Publish
    if let Err(e) = spb_edgenode_cli.publish("test".to_string(), "Hello there".to_string()) {
        println!("Error sending message: {:?}", e);
    }

    // 4. Disconnect
    spb_edgenode_cli.disconnect(None).unwrap();
}