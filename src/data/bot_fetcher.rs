use super::providers::{Provider, ProviderBot, ProviderFetchModelsResult, ProviderType};
use crate::data::providers::ProviderID;
use crate::settings::agent_view::AgentViewAction::AgentRemoved;
use crate::settings::agents::SERVER_HOST;
use crate::shared::action_notification_popup::ActionNotificationPopupAction;
use makepad_widgets::{error, Cx};
use moly_kit::agent_client::{Agent, AgentServerClient};
use moly_kit::kb_server::{KnowledgeBase, KnowledgeBaseServerClient};
use moly_kit::{protocol::*, utils::asynchronous::spawn};

/// Fetches models for a provider using the appropriate MolyKit client
pub fn fetch_models_for_provider(provider: &Provider) {
    let provider_id = provider.id.clone();
    let url = provider.url.clone();
    let api_key = provider.api_key.clone();

    match provider.provider_type {
        ProviderType::OpenAI | ProviderType::MolyServer | ProviderType::MoFa => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let mut client = moly_kit::clients::OpenAIClient::new(url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: format!("Model from {}", provider_id),
                    provider_id: provider_id.clone(),
                    enabled: true,
                },
                Some(should_include_model),
            );
        }
        ProviderType::OpenAIImage => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let client_url = url.trim_start_matches('#').to_string();
                    let mut client = moly_kit::clients::OpenAIImageClient::new(client_url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: "OpenAI Image Generation Model".to_string(),
                    provider_id: provider_id.clone(),
                    enabled: true,
                },
                None,
            );
        }
        ProviderType::OpenAIRealtime => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let client_url = url.trim_start_matches('#').to_string();
                    let mut client = moly_kit::clients::OpenAIRealtimeClient::new(client_url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: "OpenAI Realtime Model".to_string(),
                    provider_id: provider_id.clone(),
                    enabled: true,
                },
                None,
            );
        }
        ProviderType::DeepInquire => {
            fetch_models_with_client(
                provider_id.clone(),
                move || {
                    let mut client = moly_kit::clients::DeepInquireClient::new(url);
                    if let Some(key) = api_key {
                        let _ = client.set_key(&key);
                    }
                    Box::new(client)
                },
                move |bot| ProviderBot {
                    id: bot.id.clone(),
                    name: bot.name.clone(),
                    description: "A search assistant".to_string(),
                    provider_id: provider_id.clone(),
                    enabled: true,
                },
                None,
            );
        }
    }
}
pub async fn init_agents() -> Vec<Agent>{
    let client = AgentServerClient::new(format!("{}/{}",SERVER_HOST,"api/agent/fetch"));
    match client.fetch_agents().await {
        Ok(agents) => {
            agents
        }
        Err(errors) => {
            error!("初始化Agent失败{}", errors);
            vec![]
        }
    }
}

pub fn create_agent(agent: Agent){
    spawn(async move{
        let client = AgentServerClient::new(format!("{}/{}",SERVER_HOST,"api/agent/create"));
        let create_agent = client.create_agent(&agent).await;
        match create_agent {
            Ok(_) => {
                Cx::post_action(ActionNotificationPopupAction::Success("创建成功".to_string()))
            }
            Err(err) => {
                error!("{}", err);
                Cx::post_action(ActionNotificationPopupAction::Fail(format!("创建失败：{err}")))
            }
        }
    });
}

pub fn update_agent(agent: Agent){
    spawn(async move{
        let client = AgentServerClient::new(format!("{}/{}",SERVER_HOST,"api/agent/update"));
        let update_agent = client.update_agent(&agent).await;
        match update_agent {
            Ok(_) => {
                Cx::post_action(ActionNotificationPopupAction::Success("更新成功".to_string()))
            }
            Err(err) => {
                error!("{}", err);
                Cx::post_action(ActionNotificationPopupAction::Fail(format!("更新失败：{err}")))
            }
        }
    });
}

pub fn create_kb(agent: KnowledgeBase){
    spawn(async move{
        let client = KnowledgeBaseServerClient::new(format!("{}/{}",SERVER_HOST,"api/kb/create"));
        let create_agent = client.create_knowledge_base(&agent).await;
        match create_agent {
            Ok(_) => {
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    });
}

pub fn update_kb(agent: KnowledgeBase){
    spawn(async move{
        let client = KnowledgeBaseServerClient::new(format!("{}/{}",SERVER_HOST,"api/kb/update"));
        let update_agent = client.update_knowledge_base(&agent).await;
        match update_agent {
            Ok(_) => {
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    });
}

pub fn delete_kb(agent: String){
    spawn(async move{
        let client = AgentServerClient::new(format!("{}/{}/{}",SERVER_HOST,"api/sk/delete",agent));
        let delete_agent = client.delete_agent().await;
        match delete_agent {
            Ok(_) => {
                Cx::post_action(AgentRemoved);
            }
            Err(err) => {
                error!("{}", err);
            }
        }
    });
}

pub fn delete_agent(agent: String){
    spawn(async move{
        let client = AgentServerClient::new(format!("{}/{}/{}",SERVER_HOST,"api/agent/delete",agent));
        let delete_agent = client.delete_agent().await;
        match delete_agent {
            Ok(_) => {
                Cx::post_action(AgentRemoved);
                Cx::post_action(ActionNotificationPopupAction::Success("删除成功".to_string()))
            }
            Err(err) => {
                error!("{}", err);
                Cx::post_action(ActionNotificationPopupAction::Fail(format!("删除失败：{err}")))
            }
        }
    });
}


/// Generic function to fetch models using any BotClient implementation
fn fetch_models_with_client<F, M>(
    provider_id: ProviderID,
    client_factory: F,
    map_bot: M,
    filter: Option<fn(&str) -> bool>,
) where
    F: FnOnce() -> Box<dyn BotClient> + Send + 'static,
    M: Fn(Bot) -> ProviderBot + Send + 'static,
{
    spawn(async move {
        let client = client_factory();

        match client.bots().await.into_result() {
            Ok(bots) => {
                let models: Vec<ProviderBot> = bots
                    .into_iter()
                    .filter(|bot| filter.map_or(true, |f| f(&bot.name)))
                    .map(map_bot)
                    .collect();

                Cx::post_action(ProviderFetchModelsResult::Success(provider_id, models));
            }
            Err(errors) => {
                let error = if errors.is_empty() {
                    ClientError::new(
                        ClientErrorKind::Unknown,
                        "An error occurred, but no details were provided".to_string(),
                    )
                } else {
                    errors[0].clone()
                };
                Cx::post_action(ProviderFetchModelsResult::Failure(provider_id, error));
            }
        }
    });
}

/// Filter out non-chat models for OpenAI-compatible providers
fn should_include_model(model_id: &str) -> bool {
    // Filter out non-chat models
    if model_id.contains("dall-e")
        || model_id.contains("whisper")
        || model_id.contains("tts")
        || model_id.contains("davinci")
        || model_id.contains("audio")
        || model_id.contains("babbage")
        || model_id.contains("moderation")
        || model_id.contains("embedding")
    {
        return false;
    }
    true
}
