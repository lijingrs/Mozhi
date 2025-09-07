pub mod add_provider_modal;
pub mod provider_view;
pub mod providers;
pub mod agents;
pub mod providers_screen;
pub mod sync_modal;
pub mod agents_screen;
pub mod add_agent_modal;
pub mod knowledge_base_screen;
pub mod add_knowledge_base_modal;
pub mod agent_view;
pub mod knowledge_base;
pub mod knowledge_base_view;
pub mod app_center_screen;

use makepad_widgets::Cx;

pub fn live_design(cx: &mut Cx) {
    add_knowledge_base_modal::live_design(cx);
    add_provider_modal::live_design(cx);
    add_agent_modal::live_design(cx);
    app_center_screen::live_design(cx);
    knowledge_base_screen::live_design(cx);
    providers_screen::live_design(cx);
    agents_screen::live_design(cx);
    provider_view::live_design(cx);
    agent_view::live_design(cx);
    knowledge_base_view::live_design(cx);
    providers::live_design(cx);
    agents::live_design(cx);
    knowledge_base::live_design(cx);
    sync_modal::live_design(cx);
}
