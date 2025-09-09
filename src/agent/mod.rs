use makepad_widgets::Cx;

pub mod agents;
pub mod agents_screen;
pub mod agent_view;
pub mod prompt_history_screen;
pub mod add_agent_modal;

pub fn live_design(cx: &mut Cx) {
    agents_screen::live_design(cx);
    agents::live_design(cx);
    agent_view::live_design(cx);
    prompt_history_screen::live_design(cx);
    add_agent_modal::live_design(cx);
}