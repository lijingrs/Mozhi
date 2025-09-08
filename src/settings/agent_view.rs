use crate::data::bot_fetcher::{delete_agent, init_agents, update_agent};
use crate::data::store::Store;
use makepad_widgets::*;
use moly_kit::agent_client::Agent;
use moly_kit::BotId;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    REFRESH_ICON = dep("crate://self/resources/images/refresh_icon.png")

    FormGroup = <View> {
        flow: Down
        height: Fit
        spacing: 10
    }

    pub AgentView = {{AgentView}}<RoundedShadowView> {
        width: Fill, height: Fill
        visible: false
        padding: {left: 30, right: 30, top: 30, bottom: 30}
        show_bg: true
        draw_bg: {
            color: (MAIN_BG_COLOR_DARK)
            border_radius: 4.5,
            uniform shadow_color: #0002
            shadow_radius: 8.0,
            shadow_offset: vec2(0.0,-1.5)
        }

        content = <ScrollYView> {
            flow: Down, spacing: 20

            <FormGroup> {
                flow: Right,
                name = <Label> {
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 15}
                        color: #000
                    }
                }

                <View> {
                    width: Fill, height: 1
                }

                <View> {
                    align: {x: 0.5, y: 0.5}
                    width: Fit, height: Fit
                    flow: Right, spacing: 10
                    refresh_button = <View> {
                        visible: false
                        padding: {top: 4} // TODO: this is a hack to align the image view with the switch
                        cursor: Hand
                        width: 30, height: 30

                        icon = <Image> {
                            width: 22, height: 22
                            source: (REFRESH_ICON)
                        }
                    }
                    agent_enabled_switch = <MolySwitch> {
                        // Match the default value to avoid the animation on start.
                        animator: {
                            selected = {
                                default: on
                            }
                        }
                    }
                }
            }

            separator = <View> {
                height: 1,
                show_bg: true,
                draw_bg: {
                    color: #D9D9D9
                }
            }

            <FormGroup> {
                <Label> {
                    text: "App Name",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                app_name = <MolyTextInput> {
                    empty_text: "e.g. Knowledge Tracer Agent"
                }
            }

            <FormGroup> {
                <Label> {
                    text: "Agent Name",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                agent_name = <MolyTextInput> {
                    empty_text: "e.g. knowledge_tracer"
                }
            }
            <FormGroup> {
                <Label> {
                    text: "API Base"
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }

                <View> {
                    spacing: 10
                    width: Fill, height: 35
                    api_base = <MolyTextInput> {
                        width: Fill, height: 30
                        text: ""
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12}
                            color: #000
                        }
                    }
                }
            }

            // API KEY
            <FormGroup> {
                <Label> {
                    text: "API Key"
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }

                <View> {
                    spacing: 10
                    width: Fill, height: 35
                    api_key = <MolyTextInput> {
                        empty_text: ""
                        width: Fill, height: 30
                        is_password: true
                        draw_text: {
                            text_style: <REGULAR_FONT>{
                                font_size: 12
                            }
                            color: #000
                        }
                    }
                }
            }

            <FormGroup> {
                <Label> {
                    text: "Model Name",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                model_name = <MolyTextInput> {
                    empty_text: "e.g. qwen-flash"
                }
            }

            // 系统提示词区域使用带滚动的输入框
            <FormGroup> {
                <Label> {
                    text: "System Prompt",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                <ScrollYView> {
                    spacing: 10
                    width: Fill, height: 200
                    system_prompt = <MolyTextInput> {}
                }
                <View> {
                    width: Fill, height: Fit
                    align: {x: 0.0, y: 0.5}
                    connection_status = <Label> {
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 10},
                            color: #000
                        }
                    }
                }
            }

            save_agent = <MolyButton> {
                width: Fit
                height: 30
                padding: {left: 20, right: 20, top: 0, bottom: 0}
                text: "Update"
                draw_bg: { color: (CTA_BUTTON_COLOR), border_size: 0 }
            }

            remove_agent_view = <View> {
                width: Fill, height: Fit
                align: {x: 1.0, y: 0.5}
                remove_agent_button = <MolyButton> {
                    padding: {left: 20, right: 20, top: 10, bottom: 10}
                    width: Fit, height: Fit
                    text: "Remove Agent"
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 10}
                    }
                    draw_bg: { color: #B4605A, border_size: 0 }
                }
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
struct AgentView {
    #[deref]
    view: View,
    #[rust]
    agent: Agent,
}

impl Widget for AgentView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        let agent = self.agent.clone();
        if self.agent.enabled {
            self.view(id!(refresh_button)).set_visible(cx, true);
        } else {
            self.view(id!(refresh_button)).set_visible(cx, false);
        }
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, 1);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < 1 {
                        let template = live_id!(model_entry);
                        let item = list.item(cx, item_id, template);

                        // hide the separator for the first item
                        if item_id == 0 {
                            item.view(id!(separator)).set_visible(cx, false);
                        }

                        let name = agent.app_name.clone();
                        item.label(id!(model_name)).set_text(cx, &name);
                        item.check_box(id!(enabled_switch))
                            .set_active(cx, agent.enabled);

                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for AgentView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();
        // Handle agent enabled/disabled
        let agent_enabled_switch = self.check_box(id!(agent_enabled_switch));
        if let Some(enabled) = agent_enabled_switch.changed(actions) {
            self.agent.enabled = enabled;
            // Update the agent in store and preferences
            store.insert_or_update_agent(&self.agent);
            self.redraw(cx);
        }

        for action in actions {
            if let Some(action) = action.downcast_ref::<ModelEntryAction>() {
                match action {
                    ModelEntryAction::ModelEnabledChanged(model_name, enabled) => {
                        // Update the model status in the preferences
                        store.preferences.update_model_status(
                            &self.agent.id,
                            model_name,
                            *enabled,
                        );

                        // Update the model status in the store
                        if let Some(model) = store
                            .chats
                            .available_bots
                            .get_mut(&BotId::new(model_name, &self.agent.api_base))
                        {
                            model.enabled = *enabled;
                        }
                        self.redraw(cx);
                    }
                    _ => {}
                }
            }
        }

        // Handle save
        if self.button(id!(save_agent)).clicked(actions) {
            self.agent.api_base = self
                .view
                .text_input(id!(api_base))
                .text()
                .trim()
                .to_string();
            let api_key = self.view.text_input(id!(api_key)).text().trim().to_string();
            if api_key.is_empty() {
                self.agent.api_key = None;
            } else {
                self.agent.api_key = Some(api_key);
            }
            self.agent.app_name = self
                .view
                .text_input(id!(app_name))
                .text()
                .trim()
                .to_string();
            self.agent.model_name = self
                .view
                .text_input(id!(model_name))
                .text()
                .trim()
                .to_string();
            self.agent.agent_name = self
                .view
                .text_input(id!(agent_name))
                .text()
                .trim()
                .to_string();
            self.agent.system_prompt = self
                .view
                .text_input(id!(system_prompt))
                .text()
                .trim()
                .to_string();
            // Since we auto-fetch the models upon update, also enable it
            self.agent.enabled = true;
            self.check_box(id!(agent_enabled_switch))
                .set_active(cx, true);
            update_agent(self.agent.clone());
            // Update the UI

            store.insert_or_update_agent(&self.agent);
            self.redraw(cx);
        }

        // Handle refresh button
        if let Some(_fe) = self.view(id!(refresh_button)).finger_up(actions) {
            // Update the agent status in the store
            store.insert_or_update_agent(&self.agent);
            // Update UI
            self.redraw(cx);
        }

        // Handle remove agent button
        if self.button(id!(remove_agent_button)).clicked(actions) {
            store.remove_agent(&self.agent.id);
            delete_agent(self.agent.id.clone());
            if store.chats.agents.is_empty() {
                self.view.agent_view(id!(agent_view)).hide(cx);
            }
            cx.action(AgentViewAction::AgentRemoved);
            self.redraw(cx);
        }
    }
}

impl AgentViewRef {
    pub fn set_agent(&mut self, cx: &mut Cx, agent: &Agent) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.agent = agent.clone();
            inner.view.set_visible(cx, true);
            // Update the text inputs
            let api_key_input = inner.text_input(id!(api_key));
            if let Some(api_key) = &agent.api_key {
                api_key_input.set_text(cx, &api_key);
            } else {
                api_key_input.set_text(cx, "");
            }
            // api_base
            inner.text_input(id!(api_base)).set_text(cx, &agent.api_base);
            inner.text_input(id!(app_name)).set_text(cx, &agent.app_name);
            inner.text_input(id!(agent_name)).set_text(cx, &agent.agent_name);
            inner.text_input(id!(model_name)).set_text(cx, &agent.model_name);
            inner.text_input(id!(system_prompt)).set_text(cx, &agent.system_prompt);
            inner.label(id!(name)).set_text(cx, &agent.app_name);
            inner
                .check_box(id!(agent_enabled_switch))
                .set_active(cx, agent.enabled);

            inner.view(id!(remove_agent_view)).set_visible(cx, true);

            inner.view.redraw(cx);
        }
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.view.set_visible(cx, false);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum AgentViewAction {
    None,
    AgentRemoved,
    AgentAdded(Agent),
}

#[derive(Clone, Debug, DefaultNone)]
enum ModelEntryAction {
    None,
    ModelEnabledChanged(String, bool),
}
