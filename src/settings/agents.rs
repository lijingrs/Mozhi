use crate::data::store::Store;
use crate::settings::add_agent_modal::AddAgentModalAction;
use crate::settings::agent_view::AgentViewAction;
use crate::shared::actions::ChatAction;
use crate::shared::modal::ModalWidgetExt;
use makepad_widgets::*;
use moly_kit::agent_client::Agent;

pub const SERVER_HOST:&str = "http://localhost:8080";
live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    use crate::settings::add_agent_modal::*;
    use crate::shared::modal::*;

    ICON_EDIT = dep("crate://self/resources/icons/edit.svg")
    ICON_TRASH = dep("crate://self/resources/images/trash_icon.png")
    ICON_REMOTE = dep("crate://self/resources/images/globe_icon.png")
    ICON_LOCAL = dep("crate://self/resources/images/laptop_icon.png")
    ICON_SETTINGS = dep("crate://self/resources/images/settings_icon.png")

    ICON_SUCCESS = dep("crate://self/resources/images/circle_check_icon.png")
    ICON_LOADER = dep("crate://self/resources/images/loader_icon.png")
    ICON_FAILURE = dep("crate://self/resources/images/refresh_error_icon.png")

    // Agent icons
    ICON_OPENAI = dep("crate://self/resources/images/agents/openai.png")
    ICON_AGENT = dep("crate://self/resources/images/agents/agent.png")
    ICON_GEMINI = dep("crate://self/resources/images/agents/gemini.png")
    ICON_SILICONFLOW = dep("crate://self/resources/images/agents/siliconflow.png")
    ICON_OPENROUTER = dep("crate://self/resources/images/agents/openrouter.png")
    ICON_DEEPSEEK = dep("crate://self/resources/images/agents/deepseek.png")
    ICON_OLLAMA = dep("crate://self/resources/images/agents/ollama.png")
    ICON_ANTHROPIC = dep("crate://self/resources/images/agents/anthropic.png")

    // Not making this based on <Icon> because button does not support images
    // (and these SVGs are too complex for Makepad's SVG support)
    ConnectionActionButton = <View> {
        visible: false
        cursor: Hand
        width: Fit, height: Fit

        icon = <Image> {
            width: 22, height: 22
            // Override the color of the icon
            draw_bg: {
                instance tint_color: #B42318

                fn get_color_scale_pan(self, scale: vec2, pan: vec2) -> vec4 {
                    let tex_color = sample2d(self.image, self.pos * scale + pan).xyzw;
                    // Use the alpha channel from the texture but replace RGB with our tint color
                    // Assuming the icon is black/white with transparency
                    return vec4(
                        self.tint_color.rgb * tex_color.a,
                        tex_color.a
                    );
                }
            }
        }
    }

    AgentItem = {{AgentItem}}<RoundedView> {
        width: Fill, height: 40
        flow: Overlay
        show_bg: true
        draw_bg: {
            border_radius: 5
        }
        padding: {left: 20}
        align: {x: 0.0, y: 0.5}

        main_view = <View> {
            cursor: Hand
            padding: 8
            align: {x: 0.0, y: 0.5}
            spacing: 20
            flow: Right

            agent_icon = <View> {
                width: Fit, height: Fit
                image_wrapper = <View> {
                    width: Fit, height: Fit
                    agent_icon_image = <Image> {
                        width: 25, height: 25
                    }
                    visible: true
                }

                label_wrapper = <RoundedView> {
                    width: 25, height: 25
                    visible: false
                    show_bg: true
                    draw_bg: {
                        color: #344054
                        border_radius: 6
                    }
                    align: {x: 0.5, y: 0.5}

                    initial_label = <Label> {
                        draw_text:{
                            text_style: <BOLD_FONT>{font_size: 12}
                            color: #f
                        }
                    }
                }
            }

            <View> {
                flow: Right
                width: Fill, height: Fill
                spacing: 20
                align: {x: 0.0, y: 0.5}

                agent_name_label = <Label> {
                    flow: Right,
                    width: Fill,
                    draw_text:{
                        text_style: <REGULAR_FONT>{font_size: 11}
                        color: #000
                    }
                }

                // 添加Debug按钮
                debug_button = <RoundedView> {
                    cursor: Hand
                    width: Fit, height: Fit
                    padding: {left: 8, right: 8, top: 4, bottom: 4}
                    show_bg: true
                    draw_bg: {
                        color: #6366F1
                        border_radius: 4
                    }
                    align: {x: 0.5, y: 0.5}

                    <Label> {
                        text: "Run"
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 9}
                            color: #FFFFFF
                        }
                    }
                }
            }
        }
    }

    pub Agents = {{Agents}} {
        width: 300, height: Fill
        flow: Down, spacing: 10
        padding: {left: 10, right: 10}
        agents_list = <PortalList> {
            width: Fill, height: Fill
            agent_item = <AgentItem> {}
        }

        add_agent_button = <RoundedShadowView> {
            cursor: Hand
            margin: {left: 10, right: 10, bottom: 0, top: 10}
            width: Fill, height: Fit
            align: {x: 0.5, y: 0.5}
            padding: {left: 30, right: 30, bottom: 15, top: 15}
            draw_bg: {
                color: (MAIN_BG_COLOR)
                border_radius: 4.5,
                uniform shadow_color: #0002
                shadow_radius: 8.0,
                shadow_offset: vec2(0.0,-1.5)
            }
            <Label> {
                text: "+ Add Agent"
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 11}
                    color: #000
                }
            }
        }

        <View> {
            width: Fill, height: Fit
            flow: Overlay

            add_agent_modal = <Modal> {
                content: {
                    add_agent_modal_inner = <AddAgentModal> {}
                }
            }
        }
    }
}

#[derive(Widget, Live, LiveHook)]
struct Agents {
    #[deref]
    view: View,
    #[rust]
    agent_id: Option<String>,
}

impl Widget for Agents {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();

        let mut all_agents: Vec<Agent> = store.chats.agents.values().cloned().collect();
        all_agents.sort_by(|a, b| a.app_name.cmp(&b.app_name));

        let entries_count = all_agents.len();

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, entries_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < entries_count {
                        let template = live_id!(agent_item);
                        let item = list.item(cx, item_id, template);

                        // hide the separator for the first item
                        if item_id == 0 {
                            item.view(id!(separator)).set_visible(cx, false);
                        }

                        let agent = all_agents[item_id].clone();
                        let is_selected = self.agent_id == Some(agent.id.clone());
                        item.as_agent_item()
                            .set_agent(cx, agent, is_selected);
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for Agents {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // Handle modal open
        if self
            .view(id!(add_agent_button))
            .finger_up(actions)
            .is_some()
        {
            let modal = self.modal(id!(add_agent_modal));
            modal.open(cx);
        }

        for action in actions {
            // Handle selected Agent
            if let ConnectionSettingsAction::AgentSelected(agent_id) = action.cast() {
                self.agent_id = Some(agent_id);
            }

            // Handle Debug button clicked
            if let ConnectionSettingsAction::AgentDebugClicked(agent_name) = action.cast() {
                // 需要从store中获取或创建正确的BotId
                let store = scope.data.get::<Store>().unwrap();
                let bots= store.chats.available_bots.clone();
                // 查询名称为当前agent的BotId
                for (bot,provider) in bots {
                    if provider.name == agent_name{
                        cx.action(ChatAction::Start(bot));
                        break;
                    }
                }
            }
            // Handle modal actions
            if let AddAgentModalAction::ModalDismissed = action.cast() {
                self.modal(id!(add_agent_modal)).close(cx);
                self.redraw(cx);
            }

            // Handle Agent removed
            if let AgentViewAction::AgentRemoved = action.cast() {
                // Select another Agent
                let store = scope.data.get::<Store>().unwrap();
                if let Some(first_agent) = store.chats.agents.values().next() {
                    self.agent_id = Some(first_agent.id.clone());
                    cx.action(ConnectionSettingsAction::AgentSelected(
                        first_agent.id.clone(),
                    ));
                }
                self.redraw(cx);
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
struct AgentItem {
    #[deref]
    view: View,
    #[rust]
    agent: Agent,
}

impl Widget for AgentItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update the label
        self.label(id!(agent_name_label))
            .set_text(cx, &self.agent.app_name);

        self.view(id!(status_view)).set_visible(
            cx,
            true,
        );

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for AgentItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let was_item_clicked = self.view(id!(main_view)).finger_up(actions).is_some();
        if was_item_clicked {
            cx.action(ConnectionSettingsAction::AgentSelected(
                self.agent.id.clone(),
            ));
        }

        // Handle Debug button click
        let was_debug_clicked = self.view(id!(debug_button)).finger_up(actions).is_some();
        if was_debug_clicked {
            cx.action(ConnectionSettingsAction::AgentDebugClicked(
                self.agent.agent_name.clone(),
            ));
        }
    }
}

impl AgentItemRef {
    fn set_agent(
        &mut self,
        cx: &mut Cx,
        agent: Agent,
        is_selected: bool,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.agent = agent.clone();

        // Determine whether to show image or label
        // Hide the image
        inner.view(id!(image_wrapper)).set_visible(cx, false);

        // Show the label
        let label_view = inner.view(id!(label_wrapper));
        label_view.set_visible(cx, true);

        // Get first character of the Agent name
        let first_char = agent
            .app_name
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default();

        label_view
            .label(id!(initial_label))
            .set_text(cx, &first_char);

        if is_selected && cx.display_context.is_desktop() {
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { color: #EAECEF }
                },
            );
        } else {
            inner.view.apply_over(
                cx,
                live! {
                    draw_bg: { color: #f9f9f9 }
                },
            );
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ConnectionSettingsAction {
    None,
    AgentSelected(String),
    AgentDebugClicked(String),
}