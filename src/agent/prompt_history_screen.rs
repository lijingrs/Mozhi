use crate::data::store::Store;
use makepad_widgets::*;
use moly_kit::agent_client::AgentHistoryVO;
use moly_kit::PageRequest;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::widgets::*;
    use crate::shared::styles::*;

    ICON_HISTORY = dep("crate://self/resources/icons/history.png")
    ICON_ROLLBACK = dep("crate://self/resources/icons/rollback.png")
    ICON_VIEW = dep("crate://self/resources/icons/view.png")

    VersionItem = <RoundedView> {
        width: Fill,
        height: Fit,
        padding: {left: 15, right: 15, top: 12, bottom: 12},
        margin: {bottom: 8},
        show_bg: true,
        draw_bg: {
            color: #ffffff
            border_radius: 8,
            border_width: 1,
            border_color: #e0e0e0
        }

        flow: Down,
        spacing: 8,

        version_header = <View> {
            flow: Right,
            width: Fill,
            height: Fit,
            align: {x: 0.0, y: 0.5},

            version_info = <View> {
                flow: Left,
                width: Fill,
                height: Fit,
                spacing: 10,
                align: {x: 0.0, y: 0.5},

                version_badge = <RoundedView> {
                    width: Fit,
                    height: Fit,
                    padding: {left: 6, right: 6, top: 2, bottom: 2},
                    show_bg: true,
                    draw_bg: {
                        color: #6366F1
                        border_radius: 10
                    }

                    <Label> {
                        text: "v1"
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 10}
                            color: #ffffff
                        }
                    }
                }

                current_badge = <RoundedView> {
                    visible: false,
                    width: Fit,
                    height: Fit,
                    padding: {left: 6, right: 6, top: 2, bottom: 2},
                    show_bg: true,
                    draw_bg: {
                        color: #10B981
                        border_radius: 10
                    }

                    <Label> {
                        text: "Current"
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 10}
                            color: #ffffff
                        }
                    }
                }

                timestamp = <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 11}
                        color: #666
                    }
                }
            }

            actions = <View> {
                flow: Right,
                width: Fit,
                height: Fit,
                spacing: 8,

                view_button = <RoundedView> {
                    cursor: Hand,
                    width: Fit,
                    height: Fit,
                    padding: {left: 8, right: 8, top: 4, bottom: 4},
                    show_bg: true,
                    draw_bg: {
                        color: #f3f4f6
                        border_radius: 4
                    }

                    <Label> {
                        text: "View"
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 10}
                            color: #374151
                        }
                    }
                }

                rollback_button = <RoundedView> {
                    cursor: Hand,
                    width: Fit,
                    height: Fit,
                    padding: {left: 8, right: 8, top: 4, bottom: 4},
                    show_bg: true,
                    draw_bg: {
                        color: #xFEF3C7
                        border_radius: 4
                    }

                    <Label> {
                        text: "Rollback"
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 10}
                            color: #x92400E
                        }
                    }
                }
            }
        }

        version_content = <View> {
            flow: Down,
            width: Fill,
            height: Fit,
            spacing: 6,

            description = <Label> {
                visible: false,
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 11}
                    color: #666
                }
            }

            prompt_preview = <View> {
                width: Fill,
                height: Fit,
                padding: {left: 8, right: 8, top: 6, bottom: 6},
                show_bg: true,
                draw_bg: {
                    color: #f9fafb
                    border_radius: 4
                }

                <Label> {
                    text: "System prompt preview..."
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 10}
                        color: #6b7280
                    }
                }
            }
        }
    }

    pub PromptHistoryScreen = {{PromptHistoryScreen}}<RoundedShadowView> {
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

            header = <View> {
                flow: Right,
                width: Fill,
                height: Fit,
                align: {x: 0.0, y: 0.5}
                
                <Label> {
                    text: "Prompt History Versions"
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 18}
                        color: #000
                    }
                }

                <View> {
                    width: Fill,
                    height: Fit,
                }

                close_button = <RoundedView> {
                    cursor: Hand
                    width: 30, height: 30
                    align: {x: 0.5, y: 0.5}
                    show_bg: true
                    draw_bg: {
                        color: #f0f0f0
                        border_radius: 15
                    }
                    
                    <Label> {
                        text: "×"
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 20}
                            color: #666
                        }
                    }
                }
            }

            agent_info = <View> {
                flow: Down,
                width: Fill,
                height: Fit,
                spacing: 5
                
                agent_name = <Label> {
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 14}
                        color: #000
                    }
                }
                
                agent_description = <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 12}
                        color: #666
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

            versions_list = <PortalList> {
                width: Fill,
                height: Fill,
                version_item = <VersionItem> {}
            }

            empty_state = <View> {
                visible: false,
                width: Fill,
                height: Fill,
                align: {x: 0.5, y: 0.5},
                flow: Down,
                spacing: 10
                
                <Label> {
                    text: "No prompt versions found"
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 14}
                        color: #999
                    }
                }
                
                <Label> {
                    text: "Save your agent to create the first version"
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 12}
                        color: #999
                    }
                }
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
pub struct PromptHistoryScreen {
    #[deref]
    view: View,
    #[rust]
    agent_id: Option<String>,
    #[rust]
    agent_name: Option<String>,
    #[rust]
    versions: Vec<AgentHistoryVO>,
    #[rust]
    current_page: usize,
    #[rust]
    page_size: usize,
    #[rust]
    needs_data_load: bool,
    #[rust]
    pending_rollback: Option<(String, String)>,
}

impl Widget for PromptHistoryScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.page_size);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < self.versions.len() {
                        let history_vo = &self.versions[item_id];
                        let item_view = list.item(cx, item_id, live_id!(VersionItem));

                        // 设置 QuestionItem 的 ID
                        if let Some(mut version_item) = item_view.borrow_mut::<VersionItem>() {
                            version_item.version = history_vo.clone();
                        }
                        // 设置内容（截断过长文本）
                        let content = if history_vo.system_prompt.len() > 80 {
                            // 先获取字符迭代器，取前40个字符，然后重新收集为字符串
                            let first_40_chars: String = history_vo.system_prompt.chars().take(100).collect();
                            format!("{}...", first_40_chars)
                        } else {
                            history_vo.system_prompt.clone()
                        };
                        item_view.label(id!(content_label)).set_text(cx, &content);
                        item_view.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for PromptHistoryScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if let Some((agent_id, version)) = self.pending_rollback.take() {
            // TODO 异步请求 + 刷新
            self.needs_data_load = true;
        }
        
        // Load versions data if needed
        if self.needs_data_load {
            if let Some(ref agent_id) = self.agent_id {
                let store = scope.data.get::<Store>().unwrap();
                let query = PageRequest {
                    page_num: self.current_page,
                    page_size: self.page_size,
                    params: Some(agent_id.clone()),
                };
                // TODO 异步请求
                self.needs_data_load = false;
                self.redraw(cx);
            }
        }
        
        // Handle close button
        if self.view(id!(close_button)).finger_up(actions).is_some() {
            self.view.set_visible(cx, false);
        }
        
        // Handle version item actions
        for action in actions {
            if let Some(action) = action.downcast_ref::<VersionItemAction>() {
                match action {
                    VersionItemAction::None => {},
                    VersionItemAction::ViewVersion(version_id) => {
                        if let Some(version) = self.versions.iter().find(|v| v.id == *version_id) {
                            // Show version detail (could open a modal or navigate to detail view)
                            println!("View version: {}", version.id);
                        }
                    }
                    VersionItemAction::RollbackVersion(version_id) => {
                        // TODO 回滚版本
                    }
                }
            }
        }
    }
}

impl PromptHistoryScreenRef {
    pub fn set_agent(&mut self, cx: &mut Cx, agent_id: &str, agent_name: &str) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.agent_id = Some(agent_id.to_string());
            inner.agent_name = Some(agent_name.to_string());
            inner.versions = vec![]; // Reset to trigger reload
            inner.current_page = 0;
            inner.page_size = 20; // Default page size
            inner.needs_data_load = true;
            inner.pending_rollback = None;
            inner.view.set_visible(cx, true);
            
            // Update agent info directly without accessing store
            inner.view.label(id!(agent_name)).set_text(cx, agent_name);
            inner.view.label(id!(agent_description)).set_text(cx, &format!("Agent ID: {}", agent_id));
            
            inner.view.redraw(cx);
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum VersionItemAction {
    None,
    ViewVersion(String),
    RollbackVersion(String),
}

#[derive(Widget, LiveHook, Live)]
struct VersionItem {
    #[deref]
    view: View,
    #[rust]
    version: AgentHistoryVO,
}

impl Widget for VersionItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for VersionItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // Handle view button click
        if self.view(id!(view_button)).finger_up(actions).is_some() {
            cx.action(VersionItemAction::ViewVersion(
                self.version.id.clone()
            ));
        }
        
        // Handle rollback button click
        if self.view(id!(rollback_button)).finger_up(actions).is_some() {
            cx.action(VersionItemAction::RollbackVersion(
                self.version.id.clone()
            ));
        }
    }
}

impl VersionItemRef {
    pub fn set_version(&mut self, cx: &mut Cx, version: AgentHistoryVO) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.version = version.clone();
        
        // Update current badge
        let current_badge = inner.view(id!(current_badge));
        if version.is_current {
            current_badge.set_visible(cx, true);
        } else {
            current_badge.set_visible(cx, false);
        }
        
        // Update timestamp
        inner.view.label(id!(timestamp)).set_text(cx, &version.create_time);
        
        // Update description if available
        inner.view.label(id!(description)).set_text(cx, &version.system_prompt.clone());
        inner.view(id!(description)).set_visible(cx, true);
        
        // Update prompt preview
        let preview = if version.system_prompt.len() > 100 {
            format!("{}...", &version.system_prompt[..100])
        } else {
            version.system_prompt.clone()
        };
        inner.view.label(id!(prompt_preview)).set_text(cx, &preview);
        
        // Disable rollback button for current version
        if version.is_current {
            inner.view(id!(rollback_button)).set_visible(cx, false);
        } else {
            inner.view(id!(rollback_button)).set_visible(cx, true);
        }
    }
}