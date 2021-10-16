use serde::{Serialize, Deserialize};
use serde_xml_rs::{from_str};
use serde;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_snake_case)]
pub struct DeviceInfo {
    pub udn: String,
    #[serde(rename = "serial-number")]
    pub serial_number: String,
    #[serde(rename = "device-id")]
    pub device_id: String,
    #[serde(rename = "advertising-id")]
    pub advertising_id: String,
    #[serde(rename = "vendor-name")]
    pub vendor_name: String,
    #[serde(rename = "model-name")]
    pub model_name: String,
    #[serde(rename = "model-number")]
    pub model_number: String,
    #[serde(rename = "model-region")]
    pub model_region: String,
    #[serde(rename = "is-tv")]
    pub is_tv: bool,
    #[serde(rename = "is-stick")]
    pub is_stick: bool,
    #[serde(rename = "screen-size")]
    pub screen_size: u16,
    #[serde(rename = "panel-id")]
    pub panel_id: i32,
    #[serde(rename = "ui-resolution")]
    pub ui_resolution: String,
    #[serde(rename = "tuner-type")]
    pub tuner_type: String,
    #[serde(rename = "supports-ethernet")]
    pub supports_ethernet: bool,
    #[serde(rename = "wifi-mac")]
    pub wifi_mac: String,
    #[serde(rename = "wifi-driver")]
    pub wifi_driver: String,
    #[serde(rename = "has-wifi-extender")]
    pub has_wifi_extender: bool,
    #[serde(rename = "has-wifi-5G-support")]
    pub has_wifi_5G_support: bool,
    #[serde(rename = "can-use-wifi-extender")]
    pub can_use_wifi_extender: bool,
    #[serde(rename = "network-type")]
    pub network_type: String,
    #[serde(rename = "network-name")]
    pub network_name: String,
    #[serde(rename = "friendly-device-name")]
    pub friendly_device_name: String,
    #[serde(rename = "friendly-model-name")]
    pub friendly_model_name: String,
    #[serde(rename = "default-device-name")]
    pub default_device_name: String,
    #[serde(rename = "user-device-name")]
    pub user_device_name: String,
    #[serde(rename = "user-device-location")]
    pub user_device_location: Option<String>,
    #[serde(rename = "build-number")]
    pub build_number: String,
    #[serde(rename = "software-version")]
    pub software_version: String,
    #[serde(rename = "software-build")]
    pub software_build: i32,
    #[serde(rename = "secure-device")]
    pub secure_device: bool,
    pub language: String,
    pub country: String,
    pub locale: String,
    #[serde(rename = "time-zone-auto")]
    pub time_zone_auto: bool,
    #[serde(rename = "time-zone")]
    pub time_zone: String,
    #[serde(rename = "time-zone-name")]
    pub time_zone_name: String,
    #[serde(rename = "time-zone-tz")]
    pub time_zone_tz: String,
    #[serde(rename = "time-zone-offset")]
    pub time_zone_offset: String,
    #[serde(rename = "clock-format")]
    pub clock_format: String,
    pub uptime: u64,
    #[serde(rename = "power-mode")]
    pub power_mode: String,
    #[serde(rename = "supports-suspend")]
    pub supports_suspend: bool,
    #[serde(rename = "supports-find-remote")]
    pub supports_find_remote: bool,
    #[serde(rename = "supports-audio-guide")]
    pub supports_audio_guide: bool,
    #[serde(rename = "supports-rva")]
    pub supports_rva: bool,
    #[serde(rename = "developer-enabled")]
    pub developer_enabled: bool,
    #[serde(rename = "keyed-developer-id")]
    pub keyed_developer_id: Option<String>,
    #[serde(rename = "search-enabled")]
    pub search_enabled: bool,
    #[serde(rename = "search-channels-enabled")]
    pub search_channels_enabled: bool,
    #[serde(rename = "voice-search-enabled")]
    pub voice_search_enabled: bool,
    #[serde(rename = "notifications-enabled")]
    pub notifications_enabled: bool,
    #[serde(rename = "notifications-first-use")]
    pub notifications_first_use: bool, 
    #[serde(rename = "supports-private-listening")]
    pub supports_private_listening: bool,
    #[serde(rename = "supports-private-listening-dtv")]
    pub supports_private_listening_dtv: bool,
    #[serde(rename = "supports-warm-standby")]
    pub supports_warm_standby: bool,
    #[serde(rename = "headphones-connected")]
    pub headphones_connected: bool,
    #[serde(rename = "supports-ecs-textedit")]
    pub supports_ecs_textedit: bool,
    #[serde(rename = "supports-ecs-microphone")]
    pub supports_ecs_microphone: bool,
    #[serde(rename = "supports-wake-on-wlan")]
    pub supports_wake_on_wlan: bool,
    #[serde(rename = "supports-airplay")]
    pub supports_airplay: bool,
    #[serde(rename = "has-play-on-roku")]
    pub has_play_on_roku: bool,
    #[serde(rename = "has-mobile-screensaver")]
    pub has_mobile_screensaver: bool,
    #[serde(rename = "support-url")]
    pub support_url: String,
    #[serde(rename = "grandcentral-version")]
    pub grandcentral_version: String,
    #[serde(rename = "davinci-version")]
    pub davinci_version: String,
}

//fetches info on a given roku device
pub async fn fetch<'a>(device_url: &'a str) -> Result<DeviceInfo, reqwest::Error> {
    let resp = reqwest::get(format!("{}query/device-info", device_url))
        .await?
        .text()
        .await?;
    Ok(from_str(&*resp).unwrap())
}