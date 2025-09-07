use crate::data::store::Store;
use crate::settings::add_knowledge_base_modal::AddKnowledgeBaseModalAction;
use crate::settings::knowledge_base_view::KnowledgeBaseViewAction;
use crate::shared::modal::ModalWidgetExt;
use makepad_widgets::*;
use moly_kit::kb_server::KnowledgeBase;

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use link::shaders::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;
    use crate::settings::add_knowledge_base_modal::*;
    use crate::shared::modal::*;

    ICON_EDIT = dep("crate://self/resources/icons/edit.svg")
    ICON_TRASH = dep("crate://self/resources/images/trash_icon.png")
    ICON_REMOTE = dep("crate://self/resources/images/globe_icon.png")
    ICON_LOCAL = dep("crate://self/resources/images/laptop_icon.png")
    ICON_SETTINGS = dep("crate://self/resources/images/settings_icon.png")

    ICON_SUCCESS = dep("crate://self/resources/images/circle_check_icon.png")
    ICON_LOADER = dep("crate://self/resources/images/loader_icon.png")
    ICON_FAILURE = dep("crate://self/resources/images/refresh_error_icon.png")

    // knowledge_base icons
    ICON_OPENAI = dep("crate://self/resources/images/knowledge_base/openai.png")
    ICON_GEMINI = dep("crate://self/resources/images/knowledge_base/gemini.png")
    ICON_SILICONFLOW = dep("crate://self/resources/images/knowledge_base/siliconflow.png")
    ICON_OPENROUTER = dep("crate://self/resources/images/knowledge_base/openrouter.png")
    ICON_DEEPSEEK = dep("crate://self/resources/images/knowledge_base/deepseek.png")
    ICON_OLLAMA = dep("crate://self/resources/images/knowledge_base/ollama.png")
    ICON_ANTHROPIC = dep("crate://self/resources/images/knowledge_base/anthropic.png")

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

    KnowledgeBaseItem = {{KnowledgeBaseItem}}<RoundedView> {
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

            knowledge_base_icon = <View> {
                width: Fit, height: Fit
                image_wrapper = <View> {
                    width: Fit, height: Fit
                    knowledge_base_icon_image = <Image> {
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

                nowledge_base_name_label = <Label> {
                    flow: Right,
                    width: Fill,
                    draw_text:{
                        text_style: <REGULAR_FONT>{font_size: 11}
                        color: #000
                    }
                }

                status_view = <RoundedView> {
                    align: {x: 0.5, y: 0.5}
                    show_bg: true
                    width: Fit, height: Fit
                    padding: {left: 8, right: 8, bottom: 5, top: 5}
                    margin: {right: 10}
                    draw_bg: {
                        border_radius: 5
                        color: #9FD5C7
                        border_color: #357852
                        border_size: 1.2
                    }
                    status_label = <Label> {
                        text: "ON"
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 7},
                            color: #043b1c
                        }
                    }
                }
                
                // 文件上传按钮
                upload_button = <ConnectionActionButton> {
                    visible: true
                    margin: {right: 5}
                    icon = <Image> {
                        source: (ICON_EDIT),
                        width: 18, height: 18
                        draw_bg: {
                            instance tint_color: #4F46E5
                        }
                    }
                }
            }

        }

    }

    pub KnowledgeBaseWidget = {{KnowledgeBaseWidget}} {
        width: 300, height: Fill
        flow: Down, spacing: 10
        padding: {left: 10, right: 10}
        knowledge_base_list = <PortalList> {
            width: Fill, height: Fill
            knowledge_base_item = <KnowledgeBaseItem> {}
        }

        add_knowledge_base_button = <RoundedShadowView> {
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
                text: "+ Add KnowledgeBase"
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 11}
                    color: #000
                }
            }
        }

        <View> {
            width: Fill, height: Fit
            flow: Overlay

            add_knowledge_base_modal = <Modal> {
                content: {
                    add_knowledge_base_modal_inner = <AddKnowledgeBaseModal> {}
                }
            }
        }
    }
}

#[derive(Widget, Live, LiveHook)]
struct KnowledgeBaseWidget {
    #[deref]
    view: View,
    #[rust]
    knowledge_base_id: Option<String>,
}

impl Widget for KnowledgeBaseWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = scope.data.get::<Store>().unwrap();

        let mut all_knowledge_base: Vec<KnowledgeBase> = store.chats.knowledge_base.values().cloned().collect();
        all_knowledge_base.sort_by(|a, b| a.name.cmp(&b.name));

        let entries_count = all_knowledge_base.len();

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, entries_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < entries_count {
                        let template = live_id!(knowledge_base_item);
                        let item = list.item(cx, item_id, template);

                        // hide the separator for the first item
                        if item_id == 0 {
                            item.view(id!(separator)).set_visible(cx, false);
                        }

                        let knowledge_base = all_knowledge_base[item_id].clone();
                        let is_selected = self.knowledge_base_id == Some(knowledge_base.id.clone());
                        item.as_knowledge_base_item()
                            .set_knowledge_base(cx, knowledge_base, is_selected);
                        item.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for KnowledgeBaseWidget {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // Handle modal open
        if self
            .view(id!(add_knowledge_base_button))
            .finger_up(actions)
            .is_some()
        {
            let modal = self.modal(id!(add_knowledge_base_modal));
            modal.open(cx);
        }

        for action in actions {
            // Handle selected knowledge_base
            if let ConnectionSettingsAction::KnowledgeBaseSelected(knowledge_base_id) = action.cast() {
                self.knowledge_base_id = Some(knowledge_base_id);
            }

            // Handle modal actions
            if let AddKnowledgeBaseModalAction::ModalDismissed = action.cast() {
                self.modal(id!(add_knowledge_base_modal)).close(cx);
                self.redraw(cx);
            }

            // Handle knowledge_base removed
            if let KnowledgeBaseViewAction::KBRemoved = action.cast() {
                // Select another knowledge_base
                let store = scope.data.get::<Store>().unwrap();
                if let Some(first_knowledge_base) = store.chats.knowledge_base.values().next() {
                    self.knowledge_base_id = Some(first_knowledge_base.id.clone());
                    cx.action(ConnectionSettingsAction::KnowledgeBaseSelected(
                        first_knowledge_base.id.clone(),
                    ));
                } else {
                    // 没有知识库时清空选中状态
                    self.knowledge_base_id = None;
                }
                self.redraw(cx);
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
struct KnowledgeBaseItem {
    #[deref]
    view: View,
    #[rust]
    knowledge_base: KnowledgeBase,
}

impl Widget for KnowledgeBaseItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        // Update the label
        self.label(id!(knowledge_base_name_label))
            .set_text(cx, &self.knowledge_base.name);

        self.view(id!(status_view)).set_visible(
            cx,
            true,
        );

        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for KnowledgeBaseItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        let was_item_clicked = self.view(id!(main_view)).finger_up(actions).is_some();
        if was_item_clicked {
            cx.action(ConnectionSettingsAction::KnowledgeBaseSelected(
                self.knowledge_base.id.clone(),
            ));
        }
        
        // 处理上传按钮点击
        if let Some(_fe) = self.view(id!(upload_button)).finger_up(actions) {
            cx.action(ConnectionSettingsAction::FileUploadRequested(
                self.knowledge_base.id.clone(),
            ));
        }
    }
}

impl KnowledgeBaseItem {
}

impl KnowledgeBaseItemRef {
    fn set_knowledge_base(
        &mut self,
        cx: &mut Cx,
        knowledge_base: KnowledgeBase,
        is_selected: bool,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.knowledge_base = knowledge_base.clone();

        // Determine whether to show image or label
        // Hide the image
        inner.view(id!(image_wrapper)).set_visible(cx, false);

        // Show the label
        let label_view = inner.view(id!(label_wrapper));
        label_view.set_visible(cx, true);

        // Get first character of the knowledge_base name
        let first_char = knowledge_base
            .name
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
    KnowledgeBaseSelected(String),
    FileUploadRequested(String),
}
