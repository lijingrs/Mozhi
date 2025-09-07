use crate::data::bot_fetcher::{delete_kb, update_kb};
use crate::data::store::Store;
use makepad_widgets::*;
use moly_kit::kb_server::KnowledgeBase;

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

    pub KnowledgeBaseView = {{KnowledgeBaseView}}<RoundedShadowView> {
        width: Fill, height: Fill
        padding: {left: 30, right: 30, top: 30, bottom: 30}
        show_bg: true
        visible: false  // 默认隐藏，只有选中知识库时才显示
        draw_bg: {
            color: (MAIN_BG_COLOR_DARK)
            border_radius: 4.5,
            uniform shadow_color: #0002
            shadow_radius: 8.0,
            shadow_offset: vec2(0.0,-1.5)
        }

        content = <View> {
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
                    text: "Name",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                app_name = <MolyTextInput> {
                    empty_text: "e.g. question lib"
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
                    text: "Embedding Model Name",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                model_name = <MolyTextInput> {
                    empty_text: "e.g. bge-m3"
                }
            }

            // 文件上传区域
            <FormGroup> {
                <Label> {
                    text: "Upload Files",
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12}
                        color: #000
                    }
                }
                
                upload_area = <View> {
                    width: Fill, height: 100
                    show_bg: true
                    draw_bg: {
                        color: #f8f9fa
                    }
                    align: {x: 0.5, y: 0.5}
                    cursor: Hand
                    
                    <View> {
                        flow: Down
                        spacing: 10
                        align: {x: 0.5, y: 0.5}
                        
                        <Label> {
                            text: "📁 点击或拖拽文件到此处上传"
                            draw_text: {
                                text_style: <REGULAR_FONT>{font_size: 14}
                                color: #6b7280
                            }
                        }
                        
                        <Label> {
                            text: "支持 .txt, .pdf, .doc, .docx 等格式"
                            draw_text: {
                                text_style: <REGULAR_FONT>{font_size: 10}
                                color: #9ca3af
                            }
                        }
                    }
                }
                
                file_list = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 5
                    margin: {top: 10}
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
                    text: "Remove KnowledgeBase"
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
struct KnowledgeBaseView {
    #[deref]
    view: View,
    #[rust]
    kb: KnowledgeBase,
}

impl Widget for KnowledgeBaseView {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get_mut::<Store>().unwrap();
        let agent = self.kb.clone();
        self.view(id!(refresh_button)).set_visible(cx, true);
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

                        let name = agent.name.clone();
                        item.label(id!(model_name)).set_text(cx, &name);
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for KnowledgeBaseView {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let store = scope.data.get_mut::<Store>().unwrap();

        // Handle save
        if self.button(id!(save_agent)).clicked(actions) {
            self.kb.api_base = self
                .view
                .text_input(id!(api_base))
                .text()
                .trim()
                .to_string();
            let api_key = self.view.text_input(id!(api_key)).text().trim().to_string();
            if api_key.is_empty() {
                self.kb.api_key = None;
            } else {
                self.kb.api_key = Some(api_key);
            }
            self.kb.name = self
                .view
                .text_input(id!(app_name))
                .text()
                .trim()
                .to_string();
            self.kb.embedding_model = self
                .view
                .text_input(id!(model_name))
                .text()
                .trim()
                .to_string();

            update_kb(self.kb.clone());
            // Update the UI
            store.insert_or_update_kb(&self.kb);
            self.redraw(cx);
        }

        // Handle refresh button
        if let Some(_fe) = self.view(id!(refresh_button)).finger_up(actions) {
            // Update the agent status in the store
            store.insert_or_update_kb(&self.kb);

            // Update UI
            self.redraw(cx);
        }

        // Handle file upload area click
        if let Some(_fe) = self.view(id!(upload_area)).finger_up(actions) {
            // TODO: 实现文件选择对话框
            // 这里可以调用平台特定的文件选择API
            cx.action(KnowledgeBaseViewAction::FileUploadRequested);
        }

        // Handle remove agent button
        if self.button(id!(remove_agent_button)).clicked(actions) {
            store.remove_agent(&self.kb.id);
            delete_kb(self.kb.id.clone());
            cx.action(KnowledgeBaseViewAction::KBRemoved);
            self.redraw(cx);
        }
    }
}

impl KnowledgeBaseViewRef {
    pub fn set_kb(&mut self, cx: &mut Cx, agent: &KnowledgeBase) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.kb = agent.clone();

            // 显示视图
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
            inner.text_input(id!(app_name)).set_text(cx, &agent.name);
            inner.text_input(id!(model_name)).set_text(cx, &agent.embedding_model);
            inner.label(id!(name)).set_text(cx, &agent.name);

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
pub enum KnowledgeBaseViewAction {
    None,
    KBRemoved,
    FileUploadRequested,
}
