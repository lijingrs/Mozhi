use crate::data::store::Store;
use crate::settings::agent_view::AgentViewWidgetExt;
use crate::settings::agents::AgentAction;
use makepad_widgets::*;
use crate::data::bot_fetcher::init_agents;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::modal::*;
    use crate::settings::configure_connection_modal::ConfigureConnectionModal;
    use crate::settings::agent_view::AgentView;
    use crate::settings::agents::Agents;

    HorizontalSeparator = <RoundedView> {
        width: 2, height: Fill
        show_bg: true
        draw_bg: {
            color: #d3d3d3
        }
    }

    pub AgentScreen = {{AgentScreen}} {
        width: Fill, height: Fill
        spacing: 20
        flow: Down

        header = <View> {
            height: Fit
            spacing: 20
            flow: Down

            padding: {left: 30, top: 40}
            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 20}
                    color: #000
                }
                text: "Agents"
            }

            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 12}
                    color: #000
                }
                text: "Manage agents"
            }
        }

        adaptive_view = <AdaptiveView> {
            Desktop = {
                spacing: 10
                padding: {top: 10}
                agents = <Agents> {}
                agent_view = <AgentView> {}
            }

            Mobile = {
                agents = <Agents> {
                    width: Fill, height: Fill
                    padding: {left: 8, right: 8, top: 0, bottom: 0}
                }
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
pub struct AgentScreen {
    #[deref]
    view: View,
    #[rust]
    initialized: bool,
}

impl Widget for AgentScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for AgentScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let stack_navigation = self.stack_navigation(id!(navigation));
        stack_navigation.handle_stack_view_actions(cx, actions);
        if !self.initialized{
            error!("初始化Agents");
            self.init_agents();
            self.initialized = true;
        }
        for action in actions {
            if let AgentAction::AgentSelected(agent_id) = action.cast() {
                // fetch agent from store
                let agent = scope
                    .data
                    .get_mut::<Store>()
                    .unwrap()
                    .chats
                    .agents
                    .get(&agent_id);
                if let Some(agent) = agent {
                    self.view
                        .agent_view(id!(agent_view))
                        .set_agent(cx, agent);
                } else {
                    eprintln!("Agent not found: {}", agent_id);
                }
            }
        }
    }
}

impl AgentScreen {
    pub fn init_agents(&self) {
        tokio::spawn(async move {
            init_agents().await
        });
    }
}
