use crate::reply::Event;
use crate::{bail, Error};
use serde_derive::Serialize;
use serde_json::from_slice;
use std::convert::TryFrom;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Workspace,
    Mode,
    Window,
    #[serde(rename = "barconfig_update")]
    BarConfigUpdate,
    Binding,
    Shutdown,
    Tick,
    BarStateUpdate,
    Input,
}

impl TryFrom<(u32, Vec<u8>)> for Event {
    type Error = Error;

    fn try_from((event_type, payload): (u32, Vec<u8>)) -> Result<Self, Self::Error> {
        // strip the highest order bit indicating it's an event
        // since we dont convert to hex we also dont match on the (hex) values written in the sway-ipc docs!
        let event = (event_type << 1) >> 1;
        Ok(match event {
            0 => Event::Workspace(from_slice(&payload)?),
            2 => Event::Mode(from_slice(&payload)?),
            3 => Event::Window(from_slice(&payload)?),
            4 => Event::BarConfigUpdate(from_slice(&payload)?),
            5 => Event::Binding(from_slice(&payload)?),
            6 => Event::Shutdown(from_slice(&payload)?),
            7 => Event::Tick(from_slice(&payload)?),
            20 => Event::BarStateUpdate(from_slice(&payload)?),
            21 => Event::Input(from_slice(&payload)?),
            _ => bail!("received unimplemented event '{}'", event),
        })
    }
}
