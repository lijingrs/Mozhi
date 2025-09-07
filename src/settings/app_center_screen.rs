use crate::applications::app_card::AppCardClickedAction;
use crate::applications::error_notebook_app::ErrorNotebookAppAction;
use crate::applications::knowledge_graph_app::KnowledgeGraphAppAction;
use crate::applications::learn_record_app::LearnRecordAppAction;
use crate::applications::QuestionAppAction;
use crate::settings::agents::SERVER_HOST;
use crate::shared::action_notification_popup::ActionNotificationPopupAction;
use makepad_widgets::*;
use moly_kit::answer_client::AnswerClient;
use moly_kit::knowledge_client::KnowledgeClient;
use moly_kit::question_client::{QuestionServerClient, QuestionVO};
use moly_kit::PageRequest;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::shared::styles::*;
    use crate::shared::widgets::*;
    use crate::applications::question_app::QuestionApp;
    use crate::applications::error_notebook_app::ErrorNotebookApp;
    use crate::applications::learn_record_app::LearnRecordApp;
    use crate::applications::knowledge_graph_app::KnowledgeGraphApp;
    use crate::applications::app_card::AppCard;

    AppRow = <View> {
        width: Fill, height: Fit
        flow: Right
        spacing: 10
        margin: {bottom: 10}

        app1 = <AppCard> { width: Fill }
        app2 = <AppCard> { width: Fill }
        app3 = <AppCard> { width: Fill }
    }

    pub AppCenterScreen = {{AppCenterScreen}} {
        width: Fill, height: Fill
        spacing: 20
        flow: Down
        padding: {left: 30, right: 30, top: 40, bottom: 30}

        header = <View> {
            height: Fit
            spacing: 15
            flow: Down

            <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 20}
                    color: #111827
                }
                text: "AppCenter"
            }

            <Label> {
                draw_text:{
                    text_style: <REGULAR_FONT>{font_size: 14}
                    color: #6b7280
                }
                text: "Explore and use AI learning apps"
            }

            separator = <View> {
                width: Fill, height: 1
                margin: {top: 10}
                show_bg: true
                draw_bg: {
                    color: #e5e7eb
                }
            }
        }

        // 应用列表的容器
        main_content = <View> {
            width: Fill, height: Fill
            flow: Overlay

            // 默认显示的应用网格
            app_grid = <ScrollYView> {
                width: Fill, height: Fill
                visible: true

                content = <View> {
                    width: Fill
                    flow: Down
                    spacing: 5

                    // 使用PortalList来动态生成行
                    row_list = <PortalList> {
                        width: Fill
                        flow: Down
                        spacing: 5
                        AppRow = <AppRow> {}
                    }
                }
            }

            // 题目应用
            question_app_container = <View> {
                width: Fill, height: Fill
                visible: false
                flow: Down
                spacing: 10
                back_button_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {x: 0.0, y: 0.5}

                    back_btn1 = <MolyButton> {
                        width: 100, height: 36
                        text: "← 返回"
                        
                        draw_bg: {
                            color: #6b7280,
                            color_hover: #4b5563,
                            border_radius: 4.0
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #fff
                        }
                    }
                }
                question_app = <QuestionApp> {}
            },
            // 作答记录
            learn_record_app_container = <View> {
                width: Fill, height: Fill
                visible: false
                flow: Down
                spacing: 10
                back_button_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {x: 0.0, y: 0.5}

                    back_btn2 = <MolyButton> {
                        width: 100, height: 36
                        text: "← 返回"

                        draw_bg: {
                            color: #6b7280,
                            color_hover: #4b5563,
                            border_radius: 4.0
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #fff
                        }
                    }
                }
                learn_record_app = <LearnRecordApp> {}
            },
            // 知识图谱
            knowledge_graph_app_container = <View> {
                width: Fill, height: Fill
                visible: false
                flow: Down
                spacing: 10

                // 返回按钮
                back_button_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {x: 0.0, y: 0.5}

                    back_btn3 = <MolyButton> {
                        width: 100, height: 36
                        text: "← 返回"

                        draw_bg: {
                            color: #6b7280,
                            color_hover: #4b5563,
                            border_radius: 4.0
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #fff
                        }
                    }
                }
                knowledge_graph_app = <KnowledgeGraphApp> {}
            },
            // 错题本
            error_notebook_app_container = <View> {
                width: Fill, height: Fill
                visible: false
                flow: Down
                spacing: 10

                // 返回按钮
                back_button_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    align: {x: 0.0, y: 0.5}

                    back_btn4 = <MolyButton> {
                        width: 100, height: 36
                        text: "← 返回"

                        draw_bg: {
                            color: #6b7280,
                            color_hover: #4b5563,
                            border_radius: 4.0
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #fff
                        }
                    }
                }
                error_notebook_app = <ErrorNotebookApp> {}
            },
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AppInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub is_enabled: bool,
}

impl AppInfo {
    pub fn get_apps() -> Vec<AppInfo> {
        vec![
            AppInfo {
                id: "smart_question_bank".to_string(),
                name: "智慧题库".to_string(),
                description: "智能题库系统，提供个性化练习题目".to_string(),
                icon_url: None,
                is_enabled: true,
            },
            AppInfo {
                id: "knowledge_graph".to_string(),
                name: "知识图谱".to_string(),
                description: "构建和管理知识网络，发现知识间的关联".to_string(),
                icon_url: None,
                is_enabled: true,
            },
            AppInfo {
                id: "answer_record".to_string(),
                name: "学情足迹".to_string(),
                description: "记录学习过程，分析答题表现和进步轨迹".to_string(),
                icon_url: None,
                is_enabled: true,
            },
            AppInfo {
                id: "error_notebook".to_string(),
                name: "错题本".to_string(),
                description: "记录和分析错题，提供针对性练习建议".to_string(),
                icon_url: None,
                is_enabled: true,
            },
            AppInfo {
                id: "personal_portrait".to_string(),
                name: "能力画像".to_string(),
                description: "多维度可视化分析知识掌握、能力优势和薄弱环节".to_string(),
                icon_url: None,
                is_enabled: false,
            },
            AppInfo {
                id: "ling_si_note".to_string(),
                name: "灵思随记".to_string(),
                description: "智能笔记，支持语音转文字、自动整理等功能".to_string(),
                icon_url: None,
                is_enabled: false,
            },
            AppInfo {
                id: "multi_material".to_string(),
                name: "多维素材".to_string(),
                description: "多媒体素材管理，支持图片、视频、音频等".to_string(),
                icon_url: None,
                is_enabled: false,
            },
            AppInfo {
                id: "interactive_classroom".to_string(),
                name: "互动课堂".to_string(),
                description: "实时数据驱动、多模态融合AI趣味互动".to_string(),
                icon_url: None,
                is_enabled: false,
            },
            AppInfo {
                id: "virtual_lab".to_string(),
                name: "虚拟实验室".to_string(),
                description: "高仿真、可交互的虚拟实验环境".to_string(),
                icon_url: None,
                is_enabled: false,
            },
            // AppInfo {
            //     id: "study_planner".to_string(),
            //     name: "学习规划".to_string(),
            //     description: "制定个性化学习计划，跟踪学习进度".to_string(),
            //     icon_url: None,
            //     is_enabled: false,
            // },
            // AppInfo {
            //     id: "ai_tutor".to_string(),
            //     name: "AI导师".to_string(),
            //     description: "智能辅导系统，提供个性化学习指导".to_string(),
            //     icon_url: None,
            //     is_enabled: false,
            // },
            // AppInfo {
            //     id: "progress_tracker".to_string(),
            //     name: "进度追踪".to_string(),
            //     description: "实时追踪学习进度，生成学习报告".to_string(),
            //     icon_url: None,
            //     is_enabled: false,
            // },
        ]
    }

    pub fn get_initial_char(&self) -> String {
        self.name.chars().next().map(|c| c.to_string()).unwrap_or_default()
    }
}

#[derive(Widget, LiveHook, Live)]
pub struct AppCenterScreen {
    #[deref]
    view: View,
    #[rust]
    apps: Vec<AppInfo>,
    #[rust]
    current_view: AppCenterView,
    #[rust]
    is_loading_questions: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
enum AppCenterView {
    #[default]
    AppList,
    QuestionApp,
    ErrorNotebookApp,
    AnswerRecordApp,
    KnowledgeGraphApp,
}

impl Widget for AppCenterScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.apps.is_empty() {
            self.apps = AppInfo::get_apps();
        }
        // 根据当前视图状态显示不同的内容
        match self.current_view {
            AppCenterView::AppList => {
                self.view(id!(app_grid)).set_visible(cx, true);
                self.view(id!(question_app_container)).set_visible(cx, false);
                self.view(id!(learn_record_app_container)).set_visible(cx, false);
                self.view(id!(error_notebook_app_container)).set_visible(cx, false);
                self.view(id!(knowledge_graph_app_container)).set_visible(cx, false);
            }
            AppCenterView::QuestionApp => {
                self.view(id!(app_grid)).set_visible(cx, false);
                self.view(id!(header)).set_visible(cx, false);
                self.view(id!(question_app_container)).set_visible(cx, true);
                self.view(id!(learn_record_app_container)).set_visible(cx, false);
                self.view(id!(error_notebook_app_container)).set_visible(cx, false);
                self.view(id!(knowledge_graph_app_container)).set_visible(cx, false);
            },
            AppCenterView::ErrorNotebookApp => {
                self.view(id!(app_grid)).set_visible(cx, false);
                self.view(id!(header)).set_visible(cx, false);
                self.view(id!(question_app_container)).set_visible(cx, false);
                self.view(id!(learn_record_app_container)).set_visible(cx, false);
                self.view(id!(error_notebook_app_container)).set_visible(cx, true);
                self.view(id!(knowledge_graph_app_container)).set_visible(cx, false);
            },
            AppCenterView::AnswerRecordApp => {
                self.view(id!(app_grid)).set_visible(cx, false);
                self.view(id!(header)).set_visible(cx, false);
                self.view(id!(question_app_container)).set_visible(cx, false);
                self.view(id!(learn_record_app_container)).set_visible(cx, true);
                self.view(id!(error_notebook_app_container)).set_visible(cx, false);
                self.view(id!(knowledge_graph_app_container)).set_visible(cx, false);
            },
            AppCenterView::KnowledgeGraphApp => {
                self.view(id!(app_grid)).set_visible(cx, false);
                self.view(id!(header)).set_visible(cx, false);
                self.view(id!(question_app_container)).set_visible(cx, false);
                self.view(id!(learn_record_app_container)).set_visible(cx, false);
                self.view(id!(error_notebook_app_container)).set_visible(cx, false);
                self.view(id!(knowledge_graph_app_container)).set_visible(cx, true);
            },
        }
        // 计算需要多少行，每行3个应用
        let apps_count = self.apps.len();
        let rows_count = (apps_count + 2) / 3; // 向上取整

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, rows_count);

                while let Some(row_id) = list.next_visible_item(cx) {
                    if row_id >= rows_count{
                        continue;
                    }
                    let mut row_app_infos = Vec::with_capacity(3);
                    let row = list.item(cx, row_id, live_id!(AppRow));

                    // 为当前行设置最多3个应用
                    for col in 0..3 {
                        let app_index = row_id * 3 + col;
                        let card_id = match col {
                            0 => id!(app1),
                            1 => id!(app2),
                            2 => id!(app3),
                            _ => continue,
                        };

                        if app_index < apps_count {
                            let app_info = &self.apps[app_index];
                            let card = row.view(card_id);
                            let app_id = app_info.id.clone();
                            card.apply_over(cx, live!{ app_id: (app_id) });
                            // 显示卡片
                            card.set_visible(cx, true);
                            // 设置应用信息
                            card.label(id!(app_name)).set_text(cx, &app_info.name);
                            card.label(id!(app_description)).set_text(cx, &app_info.description);

                            // 设置图标或首字母
                            if app_info.icon_url.is_some() {
                                card.view(id!(icon_image)).set_visible(cx, true);
                                card.view(id!(icon_label_wrapper)).set_visible(cx, false);
                            } else {
                                let initial_char = app_info.get_initial_char();
                                card.label(id!(icon_label)).set_text(cx, &initial_char);
                                card.view(id!(icon_image)).set_visible(cx, false);
                                card.view(id!(icon_label_wrapper)).set_visible(cx, true);
                            }

                            // 设置状态
                            let status_text = if app_info.is_enabled { "可用" } else { "开发中" };
                            if !app_info.is_enabled{
                                card.label(id!(app_status)).apply_over(cx, live!{
                                    draw_text: {
                                        color: (vec4(0.6, 0.6, 0.6, 1.0))
                                    }
                                });
                            }
                            card.label(id!(app_status)).set_text(cx, status_text);
                            // 传递app_id
                            row_app_infos.push(app_info.clone());
                        } else {
                            // 隐藏多余的卡片
                            row.view(card_id).set_visible(cx, false);
                        }
                        row.draw_all(cx, scope);
                    }

                }
            }
        }
        DrawStep::done()
    }
}

impl WidgetMatchEvent for AppCenterScreen {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // 处理返回按钮点击
        if self.button(id!(back_btn1)).clicked(actions) ||self.button(id!(back_btn2)).clicked(actions) || self.button(id!(back_btn3)).clicked(actions) || self.button(id!(back_btn4)).clicked(actions) {
            self.view(id!(header)).set_visible(cx, true);
            self.show_app_list(cx);
        }

        // 处理题目应用的Action
        for action in actions {
            if let AppCardClickedAction::Clicked(app_id) = action.cast(){
                self.handle_app_selection(cx, &app_id);
            }
            if let QuestionAppAction::EditQuestion(id) = action.cast() {
                println!("在应用中心中编辑题目: {}", id);
            } else if let QuestionAppAction::DeleteQuestion(id) = action.cast() {
                println!("在应用中心中删除题目: {}", id);
            } else if let AppCenterAction::AppSelected(app_id) = action.cast() {
                println!("应用被选中: {}", app_id);
            }
        }
    }
}

impl AppCenterScreen {
    fn handle_app_selection(&mut self, cx: &mut Cx, app_id: &str) {
        match app_id {
            "smart_question_bank" => {
                self.load_question_app(cx);
            }
            "answer_record" => {
                self.load_answer_record_app(cx);
            }
            "knowledge_graph" => {
                self.load_knowledge_graph_app(cx);
            }
            "error_notebook" => {
                self.load_error_notebook_app(cx);
            }
            "ling_si_note" => {
            }
            "multi_material" => {
            }
            "study_planner" => {
            }
            "ai_tutor" => {
            }
            "progress_tracker" => {
            }
            _ => {
            }
        }
    }

    /// 加载题目应用并获取数据
    fn load_question_app(&mut self, cx: &mut Cx) {
        self.is_loading_questions = true;
        self.show_question_app(cx);
        self.fetch_questions_from_api();
        self.is_loading_questions = false;
    }

    fn load_answer_record_app(&mut self, cx: &mut Cx) {
        self.is_loading_questions = true;
        self.show_answer_record_app(cx);
        self.fetch_answer_record_from_api();
        self.is_loading_questions = false;
    }

    fn load_error_notebook_app(&mut self, cx: &mut Cx) {
        self.is_loading_questions = true;
        self.show_error_notebook_app(cx);
        self.fetch_error_notebook_from_api();
        self.is_loading_questions = false;
    }

    fn load_knowledge_graph_app(&mut self, cx: &mut Cx) {
        self.is_loading_questions = true;
        self.show_knowledge_graph_app(cx);
        self.fetch_knowledge_graph_from_api();
        self.is_loading_questions = false;
    }

    fn fetch_questions_from_api(&self) {
        tokio::spawn(async move {
            error!("正在从外部API获取题目数据...");
            let client = QuestionServerClient::new(format!("{}/{}",SERVER_HOST,"api/question/search"));
            let req = PageRequest{
                page_num: 1,
                page_size: 5,
                params: None,
            };
            let response = client.search(req).await;
            match response {
                Ok(result) => {
                    error!("检索题目返回:{:?}",result);
                    Cx::post_action(QuestionAppAction::RefreshList(result))
                }
                Err(_) => {
                    error!("搜索题目失败");
                }
            }
        });
    }


    fn fetch_answer_record_from_api(&self) {
        tokio::spawn(async move {
            error!("正在从外部API获取学习数据...");
            let client = AnswerClient::new(format!("{}/{}",SERVER_HOST,"api/learn/search"));
            let req = PageRequest{
                page_num: 1,
                page_size: 5,
                params: None,
            };
            let response = client.search(req).await;
            match response {
                Ok(result) => {
                    error!("检索学习记录返回:{:?}",result);
                    Cx::post_action(LearnRecordAppAction::RefreshList(result))
                }
                Err(err) => {
                    Cx::post_action(ActionNotificationPopupAction::Fail(format!("查询失败：{err}")));
                    error!("搜索学习记录失败");
                }
            }
        });
    }

    fn fetch_error_notebook_from_api(&self) {
        tokio::spawn(async move {
            error!("正在从外部API获取题目数据...");
            let client = AnswerClient::new(format!("{}/{}",SERVER_HOST,"api/error_notebook/search"));
            let req = PageRequest{
                page_num: 1,
                page_size: 5,
                params: None,
            };
            let response = client.search(req).await;
            match response {
                Ok(result) => {
                    error!("检索错题本返回:{:?}",result);
                    Cx::post_action(ErrorNotebookAppAction::RefreshList(result))
                }
                Err(_) => {
                    error!("搜索错题本失败");
                }
            }
        });
    }

    fn fetch_knowledge_graph_from_api(&self) {
        tokio::spawn(async move {
            error!("正在从外部API获取知识图谱数据...");
            let client = KnowledgeClient::new(format!("{}/{}",SERVER_HOST,"api/knowledge/search"));
            let req = PageRequest{
                page_num: 1,
                page_size: 5,
                params: None,
            };
            let response = client.search(req).await;
            match response {
                Ok(result) => {
                    error!("检索知识图谱返回:{:?}",result);
                    Cx::post_action(KnowledgeGraphAppAction::RefreshList(result))
                }
                Err(_) => {
                    error!("搜索知识点失败");
                }
            }
        });
    }

    /// 显示题目应用界面
    fn show_question_app(&mut self, cx: &mut Cx) {
        self.current_view = AppCenterView::QuestionApp;
        self.redraw(cx);
    }
    fn show_answer_record_app(&mut self, cx: &mut Cx) {
        self.current_view = AppCenterView::AnswerRecordApp;
        self.redraw(cx);
    }

    fn show_knowledge_graph_app(&mut self, cx: &mut Cx) {
        self.current_view = AppCenterView::KnowledgeGraphApp;
        self.redraw(cx);
    }

    fn show_error_notebook_app(&mut self, cx: &mut Cx) {
        self.current_view = AppCenterView::ErrorNotebookApp;
        self.redraw(cx);
    }

    /// 显示应用列表界面
    fn show_app_list(&mut self, cx: &mut Cx) {
        self.current_view = AppCenterView::AppList;
        self.redraw(cx);
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum AppCenterAction {
    None,
    AppSelected(String),
    QuestionAppLoaded(Vec<QuestionVO>),
    BackToAppList,
}


