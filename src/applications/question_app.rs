use crate::data::store::Store;
use crate::settings::agents::SERVER_HOST;
use crate::shared::actions::ChatAction;
use makepad_widgets::*;
use moly_kit::question_client::{QuestionSearch, QuestionServerClient, QuestionVO};
use moly_kit::{PageRequest, PageResult};

#[derive(Debug, Clone, Default)]
pub struct QuestionFilter {
    pub id_search: String,
    pub content_search: String,
    pub stage: Option<u8>, // 1: 小学, 2: 初中, 3: 高中
    pub subject: Option<String>, // "语文", "数学"
}

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    pub QuestionApp = {{QuestionApp}} {
        width: Fill,
        height: Fill,
        flow: Down,
        spacing: 15,
        padding: {left: 15, right: 15, top: 15, bottom: 15}

        show_bg: true,
        draw_bg: {
            color: #f5f5f5
        }

        // 标题
        <View> {
            width: Fill,
            height: Fit,

            <Label> {
                draw_text: {
                    text_style: <BOLD_FONT>{font_size: 16},
                    color: #333
                }
                text: "智慧题库"
            }
        }

        // 筛选面板 - 紧凑布局
        filter_panel = <RoundedView> {
            width: Fill,
            height: Fit,

            show_bg: true,
            draw_bg: {
                color: #ffffff,
            }

            padding: {left: 15, right: 15, top: 12, bottom: 12},
            flow: Down,
            spacing: 10,

            // 紧凑的筛选项布局
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 12,
                align: {x: 0, y: 0.5}

                // 题目ID搜索
                <View> {
                    width: Fit,
                    height: Fit,
                    flow: Right,
                    spacing: 8,
                    align: {x: 0, y: 0.5}
                    <Label> {
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: "ID:"
                    }

                    id_input = <MolyTextInput> {
                        width: 120,
                        height: 32,
                        align: {x: 0, y: 0.5},
                        draw_bg: {
                            border_size: 1.0
                            border_color: #ddd
                        }
                        draw_text: {
                            align: {x: 0, y: 0.5},
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #000
                            color_hover: #000
                            color_focus: #000
                            color_empty: #98A2B3
                            color_empty_focus: #98A2B3
                        }
                        text: "",
                        empty_text: ""
                    }
                }

                // 题目内容搜索
                <View> {
                    width: Fit,
                    height: Fit,
                    flow: Right,
                    spacing: 8,
                    align: {x: 0.0, y: 0.5}
                    <Label> {
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: "内容:",
                    }

                    content_input = <MolyTextInput> {
                        align: {x: 0, y: 0.5},
                        width: 120,
                        height: 32,
                        draw_bg: {
                            border_size: 1.0
                            border_color: #ddd
                        }
                        draw_text: {
                            align: {x: 0, y: 0.5},
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #000
                            color_hover: #000
                            color_focus: #000
                            color_empty: #98A2B3
                            color_empty_focus: #98A2B3
                        }
                        text: ""
                        empty_text: ""
                    }
                }

                // 学段选择 - 优化版
                <View> {
                    width: Fit,
                    height: Fit,
                    flow: Right,
                    spacing: 8,
                    align: {x: 0.0, y: 0.5},

                    <Label> {
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: "学段:"
                    }

                    stage_dropdown = <DropDown> {
                        width: 50,
                        height: 32,
                        align: {x: 0.0, y: 0.5},
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12}
                            fn get_color(self) -> vec4 {
                                return mix(
                                    #2,
                                    #x0,
                                    self.down
                                )
                            }
                        }

                        draw_bg: {
                            instance color: #f9
                            border_size: 1.0
                        }

                        labels: ["全部", "小学", "初中", "高中"]
                    }
                }

                // 学科选择 - 优化版
                <View> {
                    width: Fit,
                    height: Fit,
                    flow: Right,
                    spacing: 8,
                    align: {x: 0.0, y: 0.5}

                    <Label> {
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: "学科:"
                    }

                    subject_dropdown = <DropDown> {
                        width: 50,
                        height: 32,
                        align: {x: 0.0, y: 0.5},
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12}
                            fn get_color(self) -> vec4 {
                                return mix(
                                    #2,
                                    #x0,
                                    self.down
                                )
                            }
                        }

                        draw_bg: {
                            instance color: #f9
                            border_size: 1.0
                        }
                        labels: ["全部", "语文", "数学","英语","物理","化学","地理","政治","生物"]
                    }
                }

                // 按钮组 - 搜索和重置按钮
                <View> {
                    width: Fit,
                    height: Fit,
                    flow: Right,
                    spacing: 8,
                    align: {x: 0.0, y: 0.5}

                    search_btn = <MolyButton> {
                        width: 70,
                        height: 32,
                        text: "搜索"

                        draw_bg: {
                            color: #007bff,
                            color_hover: #0056b3,
                            color_down: #004085,
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #ffffff
                        }
                    }

                    reset_btn = <MolyButton> {
                        width: 70,
                        height: 32,
                        text: "重置"

                        draw_bg: {
                            color: #6c757d,
                            color_hover: #545b62,
                            color_down: #495057,
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #ffffff
                        }
                    }
                }

                // 添加题目按钮 - 最右边
                <View> {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    align: {x: 1.0, y: 0.5}

                    add_btn = <MolyButton> {
                        width: 80,
                        height: 32,
                        text: "添加题目"

                        draw_bg: {
                            color: #28a745,
                            color_hover: #218838,
                            color_down: #1e7e34,
                        }

                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #ffffff
                        }
                    }
                }
            }
        }

        // 题目列表面板
        questions_panel = <RoundedView> {
            width: Fill,
            height: Fill,

            show_bg: true,
            draw_bg: {
                color: #ffffff,
            }

            padding: {left: 15, right: 15, top: 15, bottom: 15},
            flow: Down,
            spacing: 10,

            // 列表标题和统计
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {x: 0.0, y: 0.5}

                <Label> {
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 14},
                        color: #333
                    }
                    text: "题目列表"
                }

                question_count = <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 12},
                        color: #666
                    }
                    text: "共0条记录"
                }
            }

            // 表头 - 更新操作列宽度
            <View> {
                width: Fill,
                height: 35,
                flow: Right,
                spacing: 10,
                padding: {left: 10, right: 10, top: 8, bottom: 8},

                show_bg: true,
                draw_bg: {
                    color: #f8f9fa,
                }

                <Label> {
                    width: 160,  // 缩小ID列
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 11},
                        color: #495057
                    }
                    text: "ID"
                }
                <Label> {
                    width: 60,
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 11},
                        color: #495057
                    }
                    text: "学科"
                }
                <Label> {
                    width: 60,
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 11},
                        color: #495057
                    }
                    text: "题型"
                }
                <Label> {
                    width: Fill,
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 11},
                        color: #495057
                    }
                    text: "题目内容"
                }
                <Label> {
                    width: 50,
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 11},
                        color: #495057
                    }
                    text: "难度"
                }
                <Label> {
                    align: {x: 0.4, y: 0.5}
                    width: 160,
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 11},
                        color: #495057
                    }
                    text: "操作"
                }
            }

            // 题目列表滚动区域
            questions_list = <PortalList> {
                width: Fill,
                height: Fill,
                flow: Down,

                QuestionItem = {{QuestionItem}} {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 10,
                    padding: {left: 10, right: 10, top: 8, bottom: 8},
                    margin: {bottom: 1}

                    show_bg: true,
                    draw_bg: {
                        color: #ffffff,
                    }

                    question_id_label = <Label> {
                        width: 160,  // 对应表头调整
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: ""
                    }

                    subject_label = <Label> {
                        width: 60,
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: ""
                    }

                     question_type_label = <Label> {
                        width: 60,
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: ""
                    }

                    content_label = <Label> {
                        width: Fill,
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057,
                            wrap: Word
                        }
                        text: ""
                    }

                    difficulty_label = <Label> {
                        width: 50,
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 11},
                            color: #495057
                        }
                        text: ""
                    }

                    // 垂直分层 + 下拉菜单操作区域
                    <View> {
                        width: 160,
                        height: Fit,
                        flow: Down,
                        spacing: 2,
                        align: {x: 0.0, y: 0.5}

                        // 第一行：问AI + 作答 + 更多操作
                        <View> {
                            width: Fill,
                            height: Fit,
                            flow: Right,
                            spacing: 1,  // 增加间距
                            margin: {top: 4, bottom: 4},  // 添加上下边距
                            align: {x: 0.0, y: 0.5},  // 垂直居中对齐

                            ai_btn = <MolyButton> {
                                width: 45,  // 稍微增加宽度
                                height: 24, // 稍微增加高度
                                text: "问AI"  // 更明确的文本

                                draw_bg: {
                                    color: #009688,
                                    color_hover: #00796B,
                                    color_down: #00695C,  // 添加按下状态
                                    border_radius: 6.0,   // 统一圆角
                                    border_size: 0.5,     // 添加细边框
                                    border_color: #00695C,
                                }

                                draw_text: {
                                    text_style: <REGULAR_FONT>{font_size: 9},  // 稍微增加字体
                                    color: #fff,
                                    color_hover: #f0f8ff,  // 悬停时字体颜色变化
                                }
                            }

                            answer_btn = <MolyButton> {
                                width: 45,
                                height: 24,
                                text: "作答"

                                draw_bg: {
                                    color: #17a2b8,
                                    color_hover: #138496,
                                    color_down: #117a8b,
                                    border_radius: 6.0,
                                    border_size: 0.5,
                                    border_color: #117a8b,
                                }

                                draw_text: {
                                    text_style: <REGULAR_FONT>{font_size: 9},
                                    color: #fff,
                                    color_hover: #f0f8ff,
                                }
                            }

                            // 更多操作下拉按钮
                            more_btn = <DropDown> {
                                width: 40,
                                height: 24,
                                align: {x: 0.0, y: 0.5},

                                draw_text: {
                                    text_style: <REGULAR_FONT>{font_size: 9},  // 稍微增加字体
                                    color: #fff,
                                    color_hover: #f0f8ff,  // 悬停时字体颜色变化
                                }

                                draw_bg: {
                                    color: #17a2b8,
                                    color_hover: #138496,
                                    color_down: #117a8b,
                                    border_radius: 6.0,
                                    border_size: 0.5,
                                    border_color: #117a8b,
                                }
                                labels: ["更多", "✏ 编辑", "🗑 删除", "📋 复制", "📤 导出"]
                                }
                                // 可选：添加分隔线
                                <View> {
                                    width: 1,
                                    height: 16,
                                    margin: {left: 4, right: 4},
                                    draw_bg: {
                                        color: #dee2e6,
                                    }
                                }
                        }
                    }
                }
            }

            // 分页控件
            pagination_panel = <Pagination>{}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct QuestionItem {
    #[deref]
    view: View,
    #[rust]
    question_id: String,
}

impl WidgetMatchEvent for QuestionItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // 问ai 跳转到题目讲解
        if self.button(id!(ai_btn)).clicked(actions) {
            let store = scope.data.get_mut::<Store>().unwrap();
            let bots = store.chats.available_bots.clone();
            let agent_name = "question_explainer";
            for (bot, provider) in bots {
                if provider.name == agent_name {
                    store.user_prompt = Some(format!("为我讲解这道题，<question_id>{}</question_id>", self.question_id));
                    cx.action(ChatAction::Start(bot));
                    break;
                }
            }
        }

        // 作答按钮
        if self.button(id!(answer_btn)).clicked(actions) {
            let store = scope.data.get_mut::<Store>().unwrap();
            let bots = store.chats.available_bots.clone();
            let agent_name = "answer_corrector";
            for (bot, provider) in bots {
                if provider.name == agent_name {
                    store.user_prompt = Some(format!("为我批改这道题，<question_id>{}</question_id>", self.question_id));
                    cx.action(ChatAction::Start(bot));
                    break;
                }
            }
        }

        // 处理下拉菜单选择
        if let Some(item) = self.drop_down(id!(more_btn)).selected(actions) {
            match item {
                1 => { // 编辑
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        QuestionAppAction::EditQuestion(self.question_id.clone())
                    );
                },
                2 => { // 删除
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        QuestionAppAction::DeleteQuestion(self.question_id.clone())
                    );
                },
                3 => { // 复制
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        QuestionAppAction::CopyQuestion(self.question_id.clone())
                    );
                },
                4 => { // 导出
                    cx.widget_action(
                        self.widget_uid(),
                        &scope.path,
                        QuestionAppAction::ExportQuestion(self.question_id.clone())
                    );
                },
                _ => {}
            }
        }
    }
}

impl Widget for QuestionItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct QuestionApp {
    #[deref]
    view: View,
    #[rust]
    questions: Vec<QuestionVO>,
    #[rust]
    filter: QuestionSearch,
    #[rust]
    current_page: usize,
    #[rust]
    page_size: usize,
    #[rust]
    total_pages: usize,
    #[rust]
    total_count: usize,
}

impl WidgetMatchEvent for QuestionApp {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // 添加题目按钮
        if self.button(id!(add_btn)).clicked(actions) {
            let agent_name = "question_structurer";
            let store = scope.data.get_mut::<Store>().unwrap();
            let bots= store.chats.available_bots.clone();
            for (bot,provider) in bots {
                if provider.name == agent_name{
                    cx.action(ChatAction::Start(bot));
                    break;
                }
            }
        }
        // 处理搜索按钮点击
        if self.button(id!(search_btn)).clicked(actions) {
            self.apply_filter(cx);
        }

        // 处理重置按钮点击
        if self.button(id!(reset_btn)).clicked(actions) {
            self.reset_filter(cx);
            self.fetch_questions_from_api();
        }

        // 处理输入框变化
        if let Some(text) = self.text_input(id!(id_input)).changed(actions) {
            self.filter.id = text;
        }

        if let Some(text) = self.text_input(id!(content_input)).changed(actions) {
            self.filter.content = Some(text);
        }

        // 处理下拉框变化
        if let Some(item) = self.drop_down(id!(stage_dropdown)).selected(actions) {
            self.filter.stage = match item {
                0 => None, // 全部
                1 => Some(1), // 小学
                2 => Some(2), // 初中
                3 => Some(3), // 高中
                _ => None,
            };
            self.apply_filter(cx);
        }

        if let Some(item) = self.drop_down(id!(subject_dropdown)).selected(actions) {
            self.filter.subject = match item {
                0 => None, // 全部
                1 => Some("语文".to_string()),
                2 => Some("数学".to_string()),
                3 => Some("英语".to_string()),
                4 => Some("物理".to_string()),
                5 => Some("化学".to_string()),
                6 => Some("地理".to_string()),
                7 => Some("政治".to_string()),
                8 => Some("生物".to_string()),
                _ => None,
            };
            self.apply_filter(cx);
        }

        // 处理每页显示数量变化
        if let Some(item) = self.drop_down(id!(page_size_dropdown)).selected(actions) {
            self.page_size = match item {
                0 => 5,
                1 => 10,
                2 => 20,
                3 => 50,
                _ => 5,
            };
            self.current_page = 1;
            self.update_pagination(cx,self.current_page,self.page_size,self.total_count);
        }

        // 处理分页按钮
        if self.button(id!(first_btn)).clicked(actions) {
            if self.current_page > 1 {
                self.current_page = 1;
                self.fetch_questions_from_api();
            }
        }

        if self.button(id!(prev_btn)).clicked(actions) {
            if self.current_page > 1 {
                self.current_page -= 1;
                self.fetch_questions_from_api();
            }
        }

        if self.button(id!(next_btn)).clicked(actions) {
            if self.current_page < self.total_pages {
                self.current_page += 1;
                self.fetch_questions_from_api();
            }
        }

        if self.button(id!(last_btn)).clicked(actions) {
            if self.current_page < self.total_pages {
                self.current_page = self.total_pages;
                self.fetch_questions_from_api();
            }
        }

        // 处理列表中的按钮点击事件
        for action in actions {
            if let QuestionAppAction::EditQuestion(id) = action.cast() {
                self.handle_question_action(cx, QuestionAppAction::EditQuestion(id));
            } else if let QuestionAppAction::DeleteQuestion(id) = action.cast() {
                self.handle_question_action(cx, QuestionAppAction::DeleteQuestion(id));
            } else if let QuestionAppAction::AnswerQuestion(id) = action.cast() {
                self.handle_question_action(cx, QuestionAppAction::AnswerQuestion(id));
            } else if let QuestionAppAction::CopyQuestion(id) = action.cast() {
                self.handle_question_action(cx, QuestionAppAction::CopyQuestion(id));
            } else if let QuestionAppAction::ExportQuestion(id) = action.cast() {
                self.handle_question_action(cx, QuestionAppAction::ExportQuestion(id));
            } else if let QuestionAppAction::RefreshList(page_result) = action.cast(){
                self.handle_question_action(cx, QuestionAppAction::RefreshList(page_result));
            }
        }
    }
}

impl Widget for QuestionApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.get_current_questions().is_empty(){
            self.init_with_sample_data();
        }
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, self.page_size);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < self.questions.len() {
                        let question = &self.questions[item_id];
                        let item_view = list.item(cx, item_id, live_id!(QuestionItem));

                        // 设置 QuestionItem 的 ID
                        if let Some(mut question_item) = item_view.borrow_mut::<QuestionItem>() {
                            question_item.question_id = question.id.clone();
                        }
                        item_view.label(id!(question_id_label)).set_text(cx, &question.id);
                        item_view.label(id!(subject_label)).set_text(cx, &format!("{}{}",question.stage,question.subject));
                        item_view.label(id!(question_type_label)).set_text(cx, &question.question_type);
                        // 设置内容（截断过长文本）
                        let content = if question.content.len() > 80 {
                            // 先获取字符迭代器，取前40个字符，然后重新收集为字符串
                            let first_40_chars: String = question.content.chars().take(100).collect();
                            format!("{}...", first_40_chars)
                        } else {
                            question.content.clone()
                        };
                        item_view.label(id!(content_label)).set_text(cx, &content);
                        item_view.label(id!(difficulty_label)).set_text(cx, &question.difficulty.to_string());
                        item_view.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl QuestionApp {
    /// 创建并初始化题目应用并加载示例数据
    pub fn with_sample_data(&mut self) {
        // 添加示例数据
        self.questions = Self::create_sample_questions();
        self.page_size = 5;
        self.total_pages = 5;
        self.current_page = 1;
    }

    /// 创建示例题目数据
    fn create_sample_questions() -> Vec<QuestionVO> {
        vec![
            QuestionVO {
                id: "Q001".to_string(),
                subject: "数学".to_string(),
                stage: "初中".to_string(), // 初中
                content: "求解一元二次方程 x² - 5x + 6 = 0 的根(默认示例数据，请勿操作！)".to_string(),
                question_type: "选择题".to_string(),
                difficulty: "简单".to_string(),
                options: Some(vec![
                    "A. x=1或x=6".to_string(),
                    "B. x=2或x=3".to_string(),
                    "C. x=-2或x=-3".to_string(),
                    "D. x=0或x=5".to_string(),
                ]),
                k_names: vec![],
            }
        ]
    }
    /// 设置题目数据
    pub fn set_questions(&mut self, questions: Vec<QuestionVO>) {
        self.questions = questions;
    }

    /// 应用筛选条件
    fn apply_filter(&mut self, cx: &mut Cx) {
        self.fetch_questions_from_api();
    }

    /// 重置筛选条件
    fn reset_filter(&mut self, cx: &mut Cx) {
        self.filter = QuestionSearch::default();
        self.current_page = 1;
        // 重置输入框
        self.text_input(id!(id_input)).set_text(cx, "");
        self.text_input(id!(content_input)).set_text(cx, "");
        self.drop_down(id!(stage_dropdown)).set_selected_item(cx,0);
        self.drop_down(id!(subject_dropdown)).set_selected_item(cx,0);
        self.update_pagination(cx,self.current_page, self.page_size,self.total_pages);
    }

    fn update_pagination(&mut self, cx: &mut Cx,page_num:usize,page_size:usize,total_count:usize) {
        let total_pages = (total_count + page_size - 1) / page_size;
        // 更新记录统计
        let count_text = format!("共{}条记录", total_count);
        self.label(id!(question_count)).set_text(cx, &count_text);
        // 更新分页信息
        let page_info = format!("第{}页 共{}页", page_num, total_pages);
        self.label(id!(pagination_info)).set_text(cx, &page_info);
        self.page_size = page_size;
        self.total_pages = total_pages;
        self.total_count = total_count;
        self.redraw(cx);
    }

    /// 应用新的题目数据并刷新显示
    pub fn refresh_questions(&mut self, cx: &mut Cx, questions: Vec<QuestionVO>) {
        self.questions = questions;
        self.apply_filter(cx);
    }

    /// 通过Action方式更新题目列表 - 这是主要的渲染接口
    pub fn handle_question_action(&mut self, cx: &mut Cx, action: QuestionAppAction) {
        match action {
            QuestionAppAction::RefreshList(page_result) => {
                self.set_questions(page_result.data);
                self.update_pagination(cx,page_result.page_num,page_result.page_size,page_result.total_count);
            },
            QuestionAppAction::FilterChanged => {
                self.apply_filter(cx);
            },
            QuestionAppAction::EditQuestion(id) => {
                println!("编辑题目: {}", id);
                // 发送编辑事件给上级组件
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    QuestionAppAction::EditQuestion(id)
                );
            },
            QuestionAppAction::DeleteQuestion(id) => {
                println!("删除题目: {}", id);
                // 实现删除逻辑
                self.questions.retain(|q| q.id != id);
                self.apply_filter(cx);

                // 发送删除事件给上级组件
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    QuestionAppAction::DeleteQuestion(id)
                );
            },
            QuestionAppAction::AnswerQuestion(id) => {
                println!("作答题目: {}", id);
                // 处理作答逻辑
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    QuestionAppAction::AnswerQuestion(id)
                );
            },
            QuestionAppAction::CopyQuestion(id) => {
                println!("复制题目: {}", id);
                // 处理复制逻辑
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    QuestionAppAction::CopyQuestion(id)
                );
            },
            QuestionAppAction::ExportQuestion(id) => {
                println!("导出题目: {}", id);
                // 处理导出逻辑
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    QuestionAppAction::ExportQuestion(id)
                );
            },
            _ => {}
        }
    }

    /// 外部调用接口：通过Action方式刷新题目列表
    pub fn refresh_by_action(&mut self, cx: &mut Cx, questions: PageResult<QuestionVO>) {
        self.handle_question_action(cx, QuestionAppAction::RefreshList(questions));
    }

    /// 外部调用接口：通过Action方式触发筛选
    pub fn filter_by_action(&mut self, cx: &mut Cx) {
        self.handle_question_action(cx, QuestionAppAction::FilterChanged);
    }
    /// 获取当前分页信息
    pub fn get_pagination_info(&self) -> (usize, usize, usize) {
        (self.current_page, self.total_pages, self.page_size)
    }
}

// 扩展的题目管理Action - 增加新的操作类型
#[derive(Clone, Debug, DefaultNone)]
pub enum QuestionAppAction {
    None,
    EditQuestion(String),
    DeleteQuestion(String),
    AnswerQuestion(String),   // 新增：作答操作
    CopyQuestion(String),     // 新增：复制操作
    ExportQuestion(String),   // 新增：导出操作
    FilterChanged,
    RefreshList(PageResult<QuestionVO>),
}

// 为外部模块提供便利的初始化方法
impl QuestionApp {
    /// 初始化题目应用并加载示例数据
    /// 这是外部使用的主要入口点
    pub fn init_with_sample_data(&mut self) {
        self.with_sample_data();
    }

    fn fetch_questions_from_api(&self) {
        let page_num = self.current_page;
        let page_size = self.page_size;
        let params = self.filter.clone();
        tokio::spawn(async move {
            error!("正在从外部API获取题目数据...");
            let client = QuestionServerClient::new(format!("{}/{}",SERVER_HOST,"api/question/search"));
            let req = PageRequest{
                page_num,
                page_size,
                params: Some(params),
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

    /// 获取当前显示的题目列表
    pub fn get_current_questions(&self) -> &Vec<QuestionVO> {
        &self.questions
    }

    /// 通过ID获取题目
    pub fn get_question_by_id(&self, id: &str) -> Option<&QuestionVO> {
        self.questions.iter().find(|q| q.id == id)
    }

    /// 更新题目
    pub fn update_question(&mut self, cx: &mut Cx, updated_question: QuestionVO) {
        if let Some(question) = self.questions.iter_mut().find(|q| q.id == updated_question.id) {
            *question = updated_question;
            self.apply_filter(cx);
        }
    }
}