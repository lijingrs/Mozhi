use makepad_widgets::*;
use moly_kit::agent_client::Agent;
use moly_kit::{protocol::ClientError, BotId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentBot {
    pub id: BotId,
    pub name: String,
    pub description: String,
    pub agent_id: String,
    pub enabled: bool,
}

impl AgentBot {
    /// Returns a dummy Agent bot whenever the corresponding Agent bot cannot be found
    /// (due to the server not being available, the server no longer providing the Agent bot, etc.).
    pub fn unknown() -> Self {
        AgentBot {
            id: BotId::new("unknown", "unknown"),
            name: "Inaccesible model - check your connections".to_string(),
            description: "This model is not currently reachable, its information is not available"
                .to_string(),
            agent_id: "unknown".to_string(),
            enabled: true,
        }
    }

    pub fn human_readable_name(&self) -> &str {
        // Trim the 'models/' prefix from Gemini models
        // TODO: also trim and cleanup naming for filenames
        self.name.trim_start_matches("models/")
    }
}

/// The connection status of the server
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub enum AgentConnectionStatus {
    #[default]
    Connecting,
    Connected,
    Disconnected,
    Error(String), // Store error message as string for serialization
}

impl AgentConnectionStatus {
    pub fn to_human_readable(&self) -> &str {
        match self {
            AgentConnectionStatus::Connecting => "Connecting...",
            AgentConnectionStatus::Connected => "Models synchronized",
            AgentConnectionStatus::Disconnected => {
                "Haven't synchronized models since app launch"
            }
            AgentConnectionStatus::Error(error_msg) => error_msg,
        }
    }

    /// Create an error status from a ClientError with a user-friendly message
    pub fn from_client_error(error: &ClientError) -> Self {
        let error_msg = error.message().to_lowercase();
        let error_string = error.to_string().to_lowercase();

        let user_message = match error.kind() {
            moly_kit::protocol::ClientErrorKind::Network => {
                if error_msg.contains("invalid url")
                    || error_msg.contains("invalid host")
                    || error_msg.contains("name resolution")
                {
                    "Invalid URL or hostname - please check your Agent configuration".to_string()
                } else if error_msg.contains("connection refused") || error_msg.contains("refused")
                {
                    "Connection refused - check if the service is running and the port is correct"
                        .to_string()
                } else if error_msg.contains("timeout") || error_msg.contains("timed out") {
                    "The server is taking too long to respond, please try again later".to_string()
                } else if error_msg.contains("ssl")
                    || error_msg.contains("tls")
                    || error_msg.contains("certificate")
                {
                    "SSL/TLS connection error - check if HTTPS is required or certificate is valid"
                        .to_string()
                } else {
                    "Network error - check your connection and URL".to_string()
                }
            }
            moly_kit::protocol::ClientErrorKind::Format => {
                "Something is wrong in our end, please file an issue if you think this is an error"
                    .to_string()
            }
            moly_kit::protocol::ClientErrorKind::Response => {
                if error_string.contains("401") || error_string.contains("unauthorized") {
                    "Unauthorized, check your API key".to_string()
                } else if error_string.contains("400") || error_string.contains("bad request") {
                    "Something is wrong in our end, please file an issue if you think this is an error".to_string()
                } else if error_string.contains("404") || error_string.contains("not found") {
                    "API endpoint not found - check your URL path".to_string()
                } else if error_string.contains("500")
                    || error_string.contains("502")
                    || error_string.contains("503")
                {
                    "We have trouble reaching the server".to_string()
                } else if error_string.contains("403") || error_string.contains("forbidden") {
                    "Access forbidden - check your API key permissions".to_string()
                } else if error_string.contains("429") || error_string.contains("rate limit") {
                    "Rate limit exceeded - please wait and try again".to_string()
                } else {
                    format!("Server error: {}", error.message())
                }
            }
            moly_kit::protocol::ClientErrorKind::Unknown => error.message().to_string(),
        };

        AgentConnectionStatus::Error(user_message)
    }
}

#[derive(Debug, DefaultNone, Clone)]
pub enum AgentFetchResult {
    Success(Vec<Agent>),
    Failure(ClientError),
    None,
}

#[derive(Live, LiveHook, PartialEq, Debug, LiveRead, Serialize, Deserialize, Clone)]
pub enum AgentType {
    #[pick]
    OpenAI,
    OpenAIImage,
    OpenAIRealtime,
    MoFa,
    DeepInquire,
    MolyServer,
}

impl Default for AgentType {
    fn default() -> Self {
        AgentType::OpenAI
    }
}
