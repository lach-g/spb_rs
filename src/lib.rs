pub use protobuf;

include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));
pub use sparkplug_b::*;
