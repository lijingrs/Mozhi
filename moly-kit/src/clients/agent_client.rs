use crate::SvrResponse;
use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct AgentServerClient(Arc<RwLock<AgentServerClientInner>>);
#[derive(Clone, Debug)]
struct AgentServerClientInner {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}
/// Represents an AI agent
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier for the Agent
    #[serde(default)]
    pub id: String,
    pub app_name: String,
    pub agent_name: String,
    pub api_base: String,
    pub api_key: Option<String>,
    pub system_prompt: String,
    pub model_name: String,
    #[serde(default)]
    pub enabled:bool,
}
impl From<AgentServerClientInner> for AgentServerClient {
    fn from(inner: AgentServerClientInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }
}
impl AgentServerClient {
    /// Creates a new client with the given DeepInquire API URL
    pub fn new(url: String) -> Self {
        let headers = HeaderMap::new();
        let client = default_client();

        AgentServerClientInner {
            url,
            headers,
            client,
        }
        .into()
    }

    pub fn set_header(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        let header_name = HeaderName::from_str(key).map_err(|_| "Invalid header name")?;

        let header_value = value.parse().map_err(|_| "Invalid header value")?;

        self.0
            .write()
            .unwrap()
            .headers
            .insert(header_name, header_value);

        Ok(())
    }
    /// 调用服务端接口获取Agents
    pub async fn fetch_agents(&self) -> Result<Vec<Agent>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.get(inner.url).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Vec<Agent>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 新增
    pub async fn create_agent(&self, params:&Agent) -> Result<Agent, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&params).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Agent>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 更新
    pub async fn update_agent(&self, params:&Agent) -> Result<Option<Agent>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&params).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Option<Agent>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 删除
    pub async fn delete_agent(&self) -> Result<Option<Agent>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.delete(inner.url).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Option<Agent>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
}


#[cfg(not(target_arch = "wasm32"))]
fn default_client() -> reqwest::Client {
    use std::time::Duration;

    reqwest::Client::builder()
        // Only considered while establishing the connection
        .connect_timeout(Duration::from_secs(360))
        // Keep high read timeout for word-by-word streaming
        .read_timeout(Duration::from_secs(360))
        .build()
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn default_client() -> reqwest::Client {
    // On web, reqwest timeouts are not configurable, but it uses the browser's
    // fetch API under the hood, which handles connection issues properly.
    reqwest::Client::new()
}
