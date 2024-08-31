// TODO move this to where it is actually used
include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
use sparkplug_b::*;

#[derive(Debug)]
pub struct MessageBuilder {
    pub topic: String,
    pub metrics: Vec<payload::Metric>,
    pub payload: Vec<u8>,
}