use crate::{PageRequest, PageResult, SvrResponse};
use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};
#[derive(Debug)]
pub struct AnswerClient(Arc<RwLock<AnswerClientInner>>);
#[derive(Clone, Debug)]
struct AnswerClientInner {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerRecordVO {
    pub id: String,
    pub question_id:String,
    pub question_type: String,
    pub subject: String,
    pub stage:String,
    pub correct_result: String,
    pub k_names: Vec<String>,
    pub create_time: String,
    pub user_answer: String,
    pub remarks:String,
}
#[derive(Debug, Clone, Serialize, Deserialize,Default)]
pub struct LearnRecordSearch {
    /// 题目学科： 数学 物理 化学...
    pub subject: Option<String>,

    /// 学段： 1 小学 2 初中 3 高中
    pub stage: Option<u8>,

    /// 题目类型: 选择题 填空题 简答题
    pub question_type: Option<String>,

    /// 知识点
    #[serde(default)]
    pub knowledge_point: Option<String>,
}
impl From<AnswerClientInner> for AnswerClient {
    fn from(inner: AnswerClientInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }
}
impl AnswerClient {
    /// Creates a new client with the given DeepInquire API URL
    pub fn new(url: String) -> Self {
        let headers = HeaderMap::new();
        let client = default_client();

        AnswerClientInner {
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
    pub async fn search(&self,req:PageRequest<LearnRecordSearch>) -> Result<PageResult<AnswerRecordVO>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&req).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<PageResult<AnswerRecordVO>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
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
