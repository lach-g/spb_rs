use paho_mqtt::ServerResponse;
use anyhow::Result;

use crate::edgenode::options::{CreateOptions, ConnectOptions, DisconnectOptions};

pub struct EdgeNode {
    client: paho_mqtt::Client
}

impl EdgeNode {
    // TODO:
    // - Maybe do the Into<> thing to support multiple input types
    pub fn new<T: Into<CreateOptions>>(opts: T) -> Result<EdgeNode> {
        let opts = opts.into();
        let cli = paho_mqtt::Client::new(opts.paho_opts)?;
        Ok(EdgeNode { client: cli})
    }

    // TODO:
    // - Maybe do the Into<> thing to support multiple input types
    // Cleaner way than branch with match?
    // ServerResponse return
    pub fn connect(&self, opt_opts: Option<ConnectOptions>) -> Result<ServerResponse> {
        match opt_opts {
            Some(opts) => Ok(self.client.connect(opts.paho_opts)?),
            None => Ok(self.client.connect(None)?)
        }
    }

    // TODO:
    // Expose MessageBuilder in a different module for spb payloads and pass as param
    // Return actual results? Pub success...?
    pub fn publish(&self, topic: String, payload: String) -> Result<()> {
        let msg = paho_mqtt::MessageBuilder::new()
            .topic(topic)
            .payload(payload.as_bytes().to_vec())
            .qos(1)
            .finalize();

        Ok(self.client.publish(msg)?)
    }

    pub fn disconnect(&self, opt_opts: Option<DisconnectOptions>) -> Result<()>  {
        match opt_opts {
            Some(opts) => Ok(self.client.disconnect(opts.paho_opts)?),
            None => Ok(self.client.disconnect(None)?)
        }
    }
}
