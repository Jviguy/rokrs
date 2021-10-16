use reqwest::Client;

pub struct Device {
    pub url: String,
    pub client: Client,
    pub info: crate::info::DeviceInfo,
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
}

pub fn new(url: String, info: crate::info::DeviceInfo) -> Device {
    let client = Client::new();
    Device {
        url,
        client,
        info,
    }
}