cfg_if::cfg_if! {
    // TODO: Maybe `json` feature flag can be avoided by using Makepad's microserde.
    if #[cfg(all(feature = "json", feature = "http"))] {
        pub mod openai;
        pub use openai::OpenAIClient;

        pub mod openai_image;
        pub use openai_image::OpenAIImageClient;

        pub mod openai_realtime;
        pub use openai_realtime::OpenAIRealtimeClient;

        pub mod deep_inquire;
        pub use deep_inquire::DeepInquireClient;
    }
}

pub use multi::*;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
pub mod multi;

pub use map::*;
pub mod map;
pub mod agent_client;
pub mod kb_server;
pub mod answer_client;
pub mod question_client;
pub mod knowledge_client;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SvrResponse<T> {
    pub code: u64,
    pub msg: Cow<'static, str>,
    pub data: Option<T>,
}


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PageRequest<T> {
    pub page_num: usize,
    pub page_size: usize,
    pub params: Option<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug,Default)]
pub struct PageResult<T> {
    pub data: Vec<T>,
    pub total_count: usize,
    pub page_num: usize,
    pub page_size: usize,
}
