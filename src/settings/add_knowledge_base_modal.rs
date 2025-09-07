use crate::data::bot_fetcher::create_kb;
use crate::data::store::Store;
use makepad_widgets::*;
use moly_kit::kb_server::KnowledgeBase;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::widgets::MolyButton;
    use crate::shared::resource_imports::*;

    FormGroup = <View> {
        flow: Down
        height: Fit
        spacing: 10
        align: {x: 0.0, y: 0.5}
    }

    ModalTextInput = <MolyTextInput> {
        draw_bg: {
            border_size: 1.0
            border_color: #ddd
        }
        draw_text: {
            text_style: <REGULAR_FONT>{font_size: 12},
            color: #000
            color_hover: #000
            color_focus: #000
            color_empty: #98A2B3
            color_empty_focus: #98A2B3
        }
        width: Fill, height: Fit
    }

    ModalLabel = <Label> {
        draw_text: {
            text_style: <REGULAR_FONT>{font_size: 12},
            color: #000
        }
    }

    pub AddKnowledgeBaseModal = {{AddKnowledgeBaseModal}} {
        width: Fit
        height: Fit

        wrapper = <RoundedView> {
            flow: Down
            width: 420
            height: 580  // 设置固定高度而不是Fit，防止无限扩展
            padding: {top: 44, right: 30 bottom: 30 left: 50}
            spacing: 10

            show_bg: true
            draw_bg: {
                color: #fff
                border_radius: 3
            }

            header = <View> {
                width: Fill,
                height: Fit,
                flow: Right
                padding: {top: 8, bottom: 20}

                title = <View> {
                    width: Fit,
                    height: Fit,

                    model_name = <Label> {
                        text: "Add KnowledgeBase",
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 13},
                            color: #000
                        }
                    }
                }

                filler_x = <View> {width: Fill, height: Fit}

                close_button = <MolyButton> {
                    width: Fit,
                    height: Fit,
                    margin: {top: -8}

                    draw_icon: {
                        svg_file: (ICON_CLOSE),
                        fn get_color(self) -> vec4 {
                            return #000;
                        }
                    }
                    icon_walk: {width: 12, height: 12}
                }
            }

            // 可滚动的内容区域
            body = <ScrollYView> {
                flow: Down
                width: Fill
                height: Fill  // 占用剩余空间
                spacing: 20
                align: {x: 0.0, y: 0.5}

                content = <View> {
                    flow: Down
                    width: Fill
                    height: Fit
                    spacing: 20

                    <FormGroup> {
                        <ModalLabel> {
                            text: "Name"
                        }
                        app_name = <ModalTextInput> {
                            empty_text: "e.g. question lib"
                        }
                    }


                    <FormGroup> {
                        <ModalLabel> {
                            text: "API Base"
                        }
                        api_base = <ModalTextInput> {
                            empty_text: "e.g. https://dashscope.aliyuncs.com/compatible-mode/v1"
                        }
                    }

                    <FormGroup> {
                        <ModalLabel> {
                            text: "API Key (optional)"
                        }
                        api_key = <ModalTextInput> {
                            empty_text: "sk-..."
                        }
                    }

                    <FormGroup> {
                        <ModalLabel> {
                            text: "Embedding Model Name"
                        }
                        model_name = <ModalTextInput> {
                            empty_text: "e.g. bge-m3"
                        }
                    }

                    error_view = <View> {
                        visible: false
                        width: Fill, height: Fit
                        error_message = <Label> {
                            draw_text: {
                                text_style: <REGULAR_FONT>{font_size: 12},
                                color: #f00
                            }
                        }
                    }

                    // 在内容底部添加一些额外间距，确保按钮不会被遮挡
                    <View> {
                        width: Fill
                        height: 20
                    }
                }
            }

            // 固定在底部的按钮区域
            footer = <View> {
                width: Fill, height: Fit
                align: {x: 1.0, y: 0.5}
                padding: {top: 15, bottom: 0}

                add_agent_button = <MolyButton> {
                    width: 150
                    height: 40
                    padding: {left: 20, right: 20, top: 0, bottom: 0}
                    text: "Save KnowledgeBase"
                    draw_bg: { color: (CTA_BUTTON_COLOR), border_color: (CTA_BUTTON_COLOR) }
                }
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum AddKnowledgeBaseModalAction {
    None,
    ModalDismissed,
}

#[derive(Live, LiveHook, Widget)]
pub struct AddKnowledgeBaseModal {
    #[deref]
    view: View,
}

impl Widget for AddKnowledgeBaseModal {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }))
    }
}

impl WidgetMatchEvent for AddKnowledgeBaseModal {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();

        if self.button(id!(close_button)).clicked(actions) {
            cx.action(AddKnowledgeBaseModalAction::ModalDismissed);
        }

        if self.button(id!(add_agent_button)).clicked(actions) {
            self.clear_error_message(cx);
            let app_name =  self.text_input(id!(app_name)).text();
            let agent_name  = self.text_input(id!(agent_name)).text();
            let api_base = self.text_input(id!(api_base)).text();
            let system_prompt = self.text_input(id!(system_prompt)).text();
            let model_name = self.text_input(id!(model_name)).text();
            if app_name.is_empty() || api_base.is_empty() || system_prompt.is_empty() || model_name.is_empty() {
                self.set_error_message(cx,"请输入必填字段内容");
                return;
            }
            // 保存之后返回KnowledgeBaseModal后更新store数据
            let agent_id = {
                let base = agent_name
                    .to_lowercase()
                    .replace(" ", "_")
                    .replace(|c: char| !c.is_alphanumeric() && c != '_', "");
                let base = if base.is_empty() {
                    "custom_provider".to_string()
                } else {
                    base
                };

                let mut id = base.clone();
                let mut counter = 1;
                while store.chats.agents.contains_key(&id) {
                    id = format!("{}_{}", base, counter);
                    counter += 1;
                }
                id
            };
            let agent = KnowledgeBase{
                id: agent_id,
                api_base,
                api_key: Some(self.text_input(id!(api_key)).text()),
                name: "".to_string(),
                embedding_model: "".to_string(),
            };
            let agent = agent.clone();
            let _= create_kb(agent.clone());
            store.insert_or_update_kb(&agent);
            cx.action(AddKnowledgeBaseModalAction::ModalDismissed);
            self.clear_form(cx);
        }
    }
}

impl AddKnowledgeBaseModal {
    fn set_error_message(&mut self, cx: &mut Cx, message: &str) {
        self.view(id!(error_view)).set_visible(cx, true);
        self.label(id!(error_message)).set_text(cx, message);
    }

    fn clear_error_message(&mut self, cx: &mut Cx) {
        self.label(id!(error_message)).set_text(cx, "");
        self.view(id!(error_view)).set_visible(cx, false);
    }

    fn clear_form(&mut self, cx: &mut Cx) {
        self.text_input(id!(api_host)).set_text(cx, "");
        self.text_input(id!(api_key)).set_text(cx, "");
        self.clear_error_message(cx);
    }
}
