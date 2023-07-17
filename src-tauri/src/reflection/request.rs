use std::time::Duration;

use mpd_client::commands;
use serde::{Deserialize, Serialize};
use specta::Type;

use super::{response::TVal, Reflect};

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub enum SingleMode {
    Enabled,
    Disabled,
    Oneshot,
}

impl Reflect for mpd_client::commands::SingleMode {
    type Output = SingleMode;
    fn reflect(self) -> Self::Output {
        match self {
            commands::SingleMode::Enabled => SingleMode::Enabled,
            commands::SingleMode::Disabled => SingleMode::Disabled,
            commands::SingleMode::Oneshot => SingleMode::Oneshot,
        }
    }
}

impl From<SingleMode> for commands::SingleMode {
    fn from(value: SingleMode) -> Self {
        match value {
            SingleMode::Enabled => commands::SingleMode::Enabled,
            SingleMode::Disabled => commands::SingleMode::Disabled,
            SingleMode::Oneshot => commands::SingleMode::Oneshot,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub enum SeekMode {
    Forward(TVal),
    Backward(TVal),
    Absolute(TVal),
}

impl From<SeekMode> for mpd_client::commands::SeekMode {
    fn from(value: SeekMode) -> Self {
        match value {
            SeekMode::Forward(v) => commands::SeekMode::Forward(Duration::from(v)),
            SeekMode::Backward(v) => commands::SeekMode::Backward(Duration::from(v)),
            SeekMode::Absolute(v) => commands::SeekMode::Absolute(Duration::from(v)),
        }
    }
}
