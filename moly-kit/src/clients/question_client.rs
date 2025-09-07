use crate::{PageRequest, PageResult, SvrResponse};
use reqwest::header::{HeaderMap, HeaderName};
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuestionSearch {
    pub id: String,
    /// 题目学科： 数学 物理 化学...
    pub subject: Option<String>,

    /// 学段： 1 小学 2 初中 3 高中
    pub stage: Option<u8>,

    /// 题目内容
    pub content: Option<String>,

    /// 题目类型: 选择题 填空题 简答题
    pub question_type: Option<String>,

    /// 难度等级： 1 简单 2 较简单 3 中等 4 较难 5 极难
    pub difficulty: Option<i32>,

    /// 关联知识点
    #[serde(default)]
    pub knowledge_points: Option<Vec<String>>,

    /// 检索第几页的内容 默认1
    pub page_num: u64,
}

#[derive(Debug)]
pub struct QuestionServerClient(Arc<RwLock<QuestionVOServerClientInner>>);
#[derive(Clone, Debug)]
struct QuestionVOServerClientInner {
    url: String,
    headers: HeaderMap,
    client: reqwest::Client,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionVO {
    pub id: String,

    /// 题目学科
    pub subject: String,

    pub stage: String,

    /// 题目内容
    pub content: String,

    /// 题目类型
    pub question_type: String,

    /// 难度等级
    pub difficulty: String,

    /// 选项列表（选择题用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,

    /// 关联知识点
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub k_names: Vec<String>,
}
impl From<QuestionVOServerClientInner> for QuestionServerClient {
    fn from(inner: QuestionVOServerClientInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }
}
impl QuestionServerClient {
    /// Creates a new client with the given DeepInquire API URL
    pub fn new(url: String) -> Self {
        let headers = HeaderMap::new();
        let client = default_client();

        QuestionVOServerClientInner {
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
    /// 调用服务端接口获取QuestionVOs
    pub async fn search(&self,req:PageRequest<QuestionSearch>) -> Result<PageResult<QuestionVO>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&req).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<PageResult<QuestionVO>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 新增
    pub async fn create_question(&self, params:&QuestionVO) -> Result<QuestionVO, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&params).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<QuestionVO>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 更新
    pub async fn update_question(&self, params:&QuestionVO) -> Result<Option<QuestionVO>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.post(inner.url).json(&params).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Option<QuestionVO>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
        Ok(response)
    }
    // 删除
    pub async fn delete_question(&self) -> Result<Option<QuestionVO>, String> {
        let inner = self.0.read().unwrap().clone();
        let client = inner.client.clone();
        let response = client.delete(inner.url).send().await.map_err(|err| err.to_string())?.text().await.map_err(|err| err.to_string())?;
        let response= serde_json::from_str::<SvrResponse<Option<QuestionVO>>>(&response).map_err(|err| err.to_string())?.data.ok_or("Invalid response")?;
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
