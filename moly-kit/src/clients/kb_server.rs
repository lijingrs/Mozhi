use crate::SvrResponse;
use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
pub struct KnowledgeBaseServerClient(Arc<RwLock<KnowledgeBaseServerClientInner>>);
#[derive(Clone, Debug)]
struct KnowledgeBaseServerClientInner {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}
/// Represents an AI KnowledgeBase
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// Unique identifier for the KnowledgeBase
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub api_key:Option<String>,
    pub api_base:String,
    pub embedding_model: String,
}
impl From<KnowledgeBaseServerClientInner> for KnowledgeBaseServerClient {
    fn from(inner: KnowledgeBaseServerClientInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }
}
impl KnowledgeBaseServerClient {
    /// Creates a new client with the given DeepInquire API URL
    pub fn new(url: String) -> Self {
        let headers = HeaderMap::new();
        let client = default_client();

        KnowledgeBaseServerClientInner {
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
    /// 调用服务端接口获取KnowledgeBases
    pub async fn fetch_knowledge_bases(&self) -> Result<Vec<KnowledgeBase>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.get(inner.url).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Vec<KnowledgeBase>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 新增
    pub async fn create_knowledge_base(&self, params:&KnowledgeBase) -> Result<KnowledgeBase, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&params).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<KnowledgeBase>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 更新
    pub async fn update_knowledge_base(&self, params:&KnowledgeBase) -> Result<Option<KnowledgeBase>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&params).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Option<KnowledgeBase>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 删除
    pub async fn delete_knowledge_base(&self) -> Result<Option<KnowledgeBase>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.delete(inner.url).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Option<KnowledgeBase>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
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
