use std::env;
use std::time::Duration;
use testcontainers::{core::{IntoContainerPort, WaitFor, Mount}, runners::AsyncRunner, GenericImage, ImageExt};

#[tokio::test]
async fn normal_publish() {
    // Setup container
    let conf_path = format!("{}\\tests\\anon.conf", env::current_dir().unwrap().display());

    let ver = "2.0.18";
    let container = GenericImage::new("eclipse-mosquitto", &ver)
        .with_exposed_port(1883.tcp())
        .with_wait_for(WaitFor::message_on_stdout(format!("mosquitto version {ver} running")))
        .with_mount(Mount::bind_mount(conf_path, "/mosquitto/config/mosquitto.conf"))
        .start()
        .await
        .expect("Mosquitto Broker should start");

    let port = container.get_host_port_ipv4(1883).await.unwrap();
    let host = format!("mqtt://localhost:{port}");
    let topic = "test/topic";
    let payload = "Hello there";

    // Setup subscriber
    let mut paho_cli = paho_mqtt::Client::new(host.as_str()).expect("Error creating Paho subscriber");

    paho_cli.set_timeout(Duration::from_secs(5));
    let rx = paho_cli.start_consuming();

    paho_cli.connect(None).expect("Error connecting Paho subscriber");
    paho_cli.subscribe(&topic, 0).expect("Error subscribing Paho subscriber");

    // Run Edge Node and publish
    let spb_edgenode_cli = spb_rs::EdgeNode::new(host).expect("Error creating Edge Node");

    spb_edgenode_cli.connect(None).expect("Error connecting Edge Node");
    spb_edgenode_cli.publish(topic.to_string(), payload.to_string()).expect("Error publishing from Edge Node");

    // Check received msg
    let msg = rx.recv().unwrap().expect("Should receive a msg");
    assert_eq!(msg.payload_str(), payload);

    // Disconnect
    spb_edgenode_cli.disconnect(None).expect("Error disconnecting Edge Node");

    if paho_cli.is_connected() {
        paho_cli.disconnect(None).expect("Error disconnecting Paho subscriber");
    }
}