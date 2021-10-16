use reqwest::Client;
use serde_xml_rs::{from_str};

pub struct Device {
    pub url: String,
    pub client: Client,
    pub info: crate::model::DeviceInfo,
}

pub type Result<T = (), E = reqwest::Error> = std::result::Result<T, E>;

impl Device {
    pub async fn keypress<'a>(&self, key: &'a str) -> Result {
        self.client.post(format!("{}keypress/{}", self.url, key)).send().await?;
        Ok(())
    }

    pub async fn launch_app<'a>(&self, app_id: &'a str) -> Result {
        self.client.post(format!("{}launch/{}", self.url, app_id)).send().await?;
        Ok(())
    }

    pub async fn install_app<'a>(&self, app_id: &'a str) -> Result {
        self.client.post(format!("{}install/{}", self.url, app_id)).send().await?;
        Ok(())
    }

    pub async fn tv_channels(&self) -> Result<crate::model::TvChannels> {
        let resp = self.client.get(format!("{}query/tv-channels", self.url))
            .send()
            .await?
            .text()
            .await?;
        Ok(from_str(&*resp).unwrap())
    }

    pub async fn active_channel(&self) -> Result<crate::model::TvChannels> {
        let resp = self.client.get(format!("{}query/tv-active-channel", self.url))
        .send()
        .await?
        .text()
        .await?;
        Ok(from_str(&*resp).unwrap())
    }
}

pub fn new(url: String, info: crate::model::DeviceInfo) -> Device {
    let client = Client::new();
    Device {
        url,
        client,
        info,
    }
}

