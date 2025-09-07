use crate::{PageRequest, PageResult, SvrResponse};
use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};
#[derive(Debug)]
pub struct KnowledgeClient(Arc<RwLock<KnowledgeClientInner>>);
#[derive(Clone, Debug)]
struct KnowledgeClientInner {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeVo {
    pub id:String,
    pub k_name:String,
    pub stage: String,
    pub subject: String,
    pub create_time: String,
    pub mastery_status: String,
}
#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct KnowledgeSearch {
    pub k_name:Option<String>,
    pub stage: Option<i8>,
    pub subject: Option<String>,
    pub mastery_status: Option<String>,
}
impl From<KnowledgeClientInner> for KnowledgeClient {
    fn from(inner: KnowledgeClientInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }
}
impl KnowledgeClient {
    /// Creates a new client with the given DeepInquire API URL
    pub fn new(url: String) -> Self {
        let headers = HeaderMap::new();
        let client = default_client();

        KnowledgeClientInner {
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
    /// 调用服务端接口获取AnswerRecordVOs
    pub async fn search(&self,req:PageRequest<KnowledgeSearch>) -> Result<PageResult<KnowledgeVo>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&req).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<PageResult<KnowledgeVo>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
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
