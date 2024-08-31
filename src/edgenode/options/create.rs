pub struct CreateOptions {
    pub (crate) paho_opts: paho_mqtt::CreateOptions
}

impl From<String> for CreateOptions {
    fn from(server_uri: String) -> Self {
        let paho_opts = paho_mqtt::CreateOptionsBuilder::new()
            .server_uri(server_uri)
            .finalize();
        Self { paho_opts }
    }
}
