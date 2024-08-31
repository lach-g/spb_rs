use paho_mqtt;
use std::{env, process, time::Duration};

fn main() {
    // Create a client & define connect options
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| "mqtt://localhost:1883".to_string());

    // 1. Configure client
    let mut paho_cli = paho_mqtt::Client::new(host.as_str()).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    // Use 5sec timeouts for sync calls.
    paho_cli.set_timeout(Duration::from_secs(5));

    // 2. Connect
    if let Err(e) = paho_cli.connect(None) {
        println!("Unable to connect: {:?}", e);
        process::exit(1);
    }

    // 3. Build Message
    let msg = paho_mqtt::MessageBuilder::new()
        .topic("test")
        .payload("Hello synchronous world!")
        .qos(1)
        .finalize();

    // 4. Publish
    if let Err(e) = paho_cli.publish(msg) {
        println!("Error sending message: {:?}", e);
    }

    // 4. Disconnect
    paho_cli.disconnect(None).unwrap();
}
