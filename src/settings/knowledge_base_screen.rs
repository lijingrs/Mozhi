use crate::data::store::Store;
use crate::settings::knowledge_base::ConnectionSettingsAction;
use crate::settings::knowledge_base_view::{KnowledgeBaseViewAction, KnowledgeBaseViewWidgetExt};
use makepad_widgets::*;
use moly_kit::kb_server::KnowledgeBase;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::shared::modal::*;
    use crate::settings::configure_connection_modal::ConfigureConnectionModal;
    use crate::settings::knowledge_base_view::KnowledgeBaseView;
    use crate::settings::knowledge_base::KnowledgeBaseWidget;

    HorizontalSeparator = <RoundedView> {
        width: 2, height: Fill
        show_bg: true
        draw_bg: {
            color: #d3d3d3
        }
    }

    pub KnowledgeBaseScreen = {{KnowledgeBaseScreen}} {
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
                text: "KnowledgeBase"
            }

            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 12}
                    color: #000
                }
                text: "Manage KnowledgeBase"
            }
        }

        adaptive_view = <AdaptiveView> {
            Desktop = {
                spacing: 10
                padding: {top: 10}
                knowledgeBase = <KnowledgeBaseWidget> {}
                agent_view = <KnowledgeBaseView> {}
            }

            Mobile = {
                knowledgeBase = <KnowledgeBaseWidget> {
                    width: Fill, height: Fill
                    padding: {left: 8, right: 8, top: 0, bottom: 0}
                }
            }
        }
    }
}

#[derive(Widget, LiveHook, Live)]
pub struct KnowledgeBaseScreen {
    #[deref]
    view: View,
    #[rust]
    knowledge_bases: Vec<KnowledgeBase>,
}

impl Widget for KnowledgeBaseScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for KnowledgeBaseScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        let stack_navigation = self.stack_navigation(id!(navigation));
        stack_navigation.handle_stack_view_actions(cx, actions);

        for action in actions {
            if let ConnectionSettingsAction::KnowledgeBaseSelected(agent_id) = action.cast() {
                // fetch agent from store
                let agent = scope
                    .data
                    .get_mut::<Store>()
                    .unwrap()
                    .chats
                    .knowledge_base
                    .get(&agent_id);
                if let Some(agent) = agent {
                    self.view
                        .knowledge_base_view(id!(agent_view))
                        .set_kb(cx, agent);
                } else {
                    eprintln!("Agent not found: {}", agent_id);
                }
            }
            
            // 处理知识库被删除的情况
            if let KnowledgeBaseViewAction::KBRemoved = action.cast() {
                // 检查是否还有其他知识库
                let store = scope.data.get::<Store>().unwrap();
                if store.chats.knowledge_base.is_empty() {
                    // 没有知识库时隐藏视图
                    self.view.knowledge_base_view(id!(agent_view)).hide(cx);
                }
            }
            
            // 处理文件上传请求
            if let KnowledgeBaseViewAction::FileUploadRequested = action.cast() {
                // TODO: 实现文件上传逻辑
                println!("文件上传请求被触发");
            }
            
            // 处理知识库项的文件上传请求
            if let ConnectionSettingsAction::FileUploadRequested(kb_id) = action.cast() {
                // TODO: 实现针对特定知识库的文件上传逻辑
                println!("为知识库 {} 上传文件", kb_id);
            }
        }
    }
}
