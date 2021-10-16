use serde::{Serialize, Deserialize};
use serde;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct TvChannels {
    #[serde(rename(deserialize="$value"), default)]
    pub channels: Vec<Channel>
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct TvChannel {
    pub channel: Option<ActiveChannel>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct Channel {
    pub number: f64,
    pub name: String,
    pub r#type: String,
    #[serde(rename = "user-hidden")]
    pub user_hidden: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[allow(non_snake_case)]
pub struct ActiveChannel {
    pub number: f64,
    pub name: String,
    pub r#type: String,
    #[serde(rename = "user-hidden")]
    pub user_hidden: bool,
    #[serde(rename = "active-input")]
    pub active_input: bool,
    #[serde(rename = "signal-state")]
    pub signal_state: String,
    #[serde(rename = "signal-mode")]
    pub signal_mode: String,
    #[serde(rename = "signal-quality")]
    pub signal_quality: u64,
    #[serde(rename = "signal-strength")]
    pub signal_strength: i64,
    #[serde(rename = "program-title")]
    pub program_title: String,
    #[serde(rename = "program-description")]
    pub program_description: String,
    #[serde(rename = "program-ratings")]
    pub program_ratings: String,
    #[serde(rename = "program-analog-audio")]
    pub program_analog_audio: String,
    #[serde(rename = "program-digital-audio")]
    pub program_digital_audio: String,
    #[serde(rename = "program-audio-languages")]
    pub program_audio_languages: String,
    #[serde(rename = "program-audio-formats")]
    pub program_audio_formats: String,
    #[serde(rename = "program-audio-language")]
    pub program_audio_language: String,
    #[serde(rename = "program-audio-format")]
    pub program_audio_format: String,
    #[serde(rename = "program-has-cc")]
    pub program_has_cc: bool,
}