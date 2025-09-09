use crate::data::store::Store;
use crate::agent::agents::SERVER_HOST;
use crate::shared::actions::ChatAction;
use makepad_widgets::*;
use moly_kit::answer_client::{AnswerClient, AnswerRecordVO, LearnRecordSearch};
use moly_kit::{PageRequest, PageResult};
use crate::shared::action_notification_popup::ActionNotificationPopupAction;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::widgets::*;

    pub ErrorNotebookApp = {{ErrorNotebookApp}} {
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
                text: "错题本"
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
                        text: "知识点:"
                    }

                    k_name_input = <MolyTextInput> {
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
                        text: "题型:"
                    }

                    question_type_input = <MolyTextInput> {
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
            }
        }

        // 题目列表面板
        answers_panel = <RoundedView> {
            width: Fill,
            height: Fill,

            show_bg: true,
            draw_bg: {
                color: #ffffff,
            }

            padding: {left: 15, right: 15, top: 15, bottom: 15},
            flow: Down,
            spacing: 8,

            // 列表标题和统计
            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 12,
                align: {x: 0.0, y: 0.5}

                <Label> {
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 16},
                        color: #333
                    }
                    text: "错题列表"
                }

                question_count = <Label> {
                    draw_text: {
                        text_style: <REGULAR_FONT>{font_size: 13},
                        color: #666
                    }
                    text: "共0条记录"
                }
            }

            // 表头 - 优化列宽，充分利用空间
            <View> {
                width: Fill,
                height: 40,
                flow: Right,
                spacing: 8,
                padding: {left: 12, right: 12, top: 10, bottom: 10},

                show_bg: true,
                draw_bg: {
                    color: #f8f9fa,
                }

                <Label> {
                    width: 120,  // 题ID适当增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057
                    }
                    text: "题ID"
                }
                <Label> {
                    width: 80,   // 题型增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057
                    }
                    text: "题型"
                }
                <Label> {
                    width: 60,   // 学科保持适中
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057
                    }
                    text: "学科"
                }
                <Label> {
                    width: 90,   // 批改结果增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057
                    }
                    text: "批改结果"
                }
                <Label> {
                    width: 200,  // 知识点大幅增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057,
                        wrap: Word
                    }
                    text: "知识点"
                }
                <Label> {
                    width: 180,  // 作答内容大幅增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057,
                        wrap: Word
                    }
                    text: "作答内容"
                }
                <Label> {
                    width: Fill,  // 批改内容使用剩余所有空间
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057,
                        wrap: Word
                    }
                    text: "批改内容"
                }
                <Label> {
                    width: 160,  // 作答时间增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057
                    }
                    text: "作答时间"
                }
                <Label> {
                    align: {x: 0.5, y: 0.5}
                    width: 110,  // 操作区域适当增加
                    draw_text: {
                        text_style: <BOLD_FONT>{font_size: 12},
                        color: #495057
                    }
                    text: "操作"
                }
            }

            // 列表滚动区域
            answers_list = <PortalList> {
                width: Fill,
                height: Fill,
                flow: Down,

                AnswerItem = {{AnswerItem}} {
                    width: Fill,
                    height: Fit,
                    flow: Right,
                    spacing: 8,
                    padding: {left: 12, right: 12, top: 12, bottom: 12},
                    margin: {bottom: 2}

                    show_bg: true,
                    draw_bg: {
                        color: #ffffff,
                    }

                    question_id_label = <Label> {
                        width: 120,  // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057
                        }
                        text: ""
                    }

                    question_type_label = <Label> {
                        width: 80,   // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057
                        }
                        text: ""
                    }

                    subject_label = <Label> {
                        width: 60,   // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057
                        }
                        text: ""
                    }

                    correct_result_label = <Label> {
                        width: 90,   // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057
                        }
                        text: ""
                    }

                    k_names_label = <Label> {
                        width: 200,  // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057,
                            wrap: Word
                        }
                        text: ""
                    }

                    content_label = <Label> {
                        width: 180,  // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057,
                            wrap: Word
                        }
                        text: ""
                    }

                    user_answer_label = <Label> {
                        width: Fill,  // 与头部对应，使用剩余所有空间
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057,
                            wrap: Word
                        }
                        text: ""
                    }

                    create_time_label = <Label> {
                        width: 160,  // 与头部对应
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12},
                            color: #495057
                        }
                        text: ""
                    }

                    <View> {
                        width: 110,  // 与头部对应
                        height: Fit,
                        flow: Right,
                        spacing: 6,
                        align: {x: 0.5, y: 0.5}

                        ai_btn = <MolyButton> {
                            width: 50,   // 按钮适当增加
                            height: 26,  // 高度增加
                            text: "问AI"

                            draw_bg: {
                                color: #009688,
                                color_hover: #00796B,
                            }

                            draw_text: {
                                text_style: <REGULAR_FONT>{font_size: 10},
                                color: #fff
                            }
                        }

                        delete_btn = <MolyButton> {
                            width: 50,   // 按钮适当增加
                            height: 26,  // 高度增加
                            text: "删除"

                            draw_bg: {
                                color: #dc3545,
                                color_hover: #c82333,
                            }

                            draw_text: {
                                text_style: <REGULAR_FONT>{font_size: 10},
                                color: #fff
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
pub struct AnswerItem {
    #[deref]
    view: View,
    #[rust]
    question_id: String,
    #[rust]
    id: String,
}

impl WidgetMatchEvent for AnswerItem {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        // 问ai 跳转到数据报告
        if self.button(id!(ai_btn)).clicked(actions) {
            let store = scope.data.get_mut::<Store>().unwrap();
            let bots= store.chats.available_bots.clone();
            let agent_name = "error_analyzer";
            for (bot,provider) in bots {
                if provider.name == agent_name{
                    store.user_prompt = Some(format!("对当前作答数据进行错因分析，<answer_id>{}</answer_id>",self.id));
                    cx.action(ChatAction::Start(bot));
                    break;
                }
            }
        }
        if self.button(id!(edit_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ErrorNotebookAppAction::EditQuestion(self.question_id.clone())
            );
        }

        if self.button(id!(delete_btn)).clicked(actions) {
            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ErrorNotebookAppAction::DeleteQuestion(self.question_id.clone())
            );
        }
    }
}

impl Widget for AnswerItem {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ErrorNotebookApp {
    #[deref]
    view: View,
    #[rust]
    answers: Vec<AnswerRecordVO>,
    #[rust]
    filter: LearnRecordSearch,
    #[rust]
    current_page: usize,
    #[rust]
    page_size: usize,
    #[rust]
    total_pages: usize,
    #[rust]
    total_count: usize,
}

impl WidgetMatchEvent for ErrorNotebookApp {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        // 处理搜索按钮点击
        if self.button(id!(search_btn)).clicked(actions) {
            self.apply_filter(cx);
        }

        // 处理重置按钮点击
        if self.button(id!(reset_btn)).clicked(actions) {
            self.reset_filter(cx);
            self.fetch_error_notebook_from_api();
        }

        // 处理输入框变化
        if let Some(text) = self.text_input(id!(k_name_input)).changed(actions) {
            self.filter.knowledge_point = Some(text);
        }

        if let Some(text) = self.text_input(id!(question_type_input)).changed(actions) {
            self.filter.question_type = Some(text);
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
                self.fetch_error_notebook_from_api();
            }
        }

        if self.button(id!(prev_btn)).clicked(actions) {
            if self.current_page > 1 {
                self.current_page -= 1;
                self.fetch_error_notebook_from_api();
            }
        }

        if self.button(id!(next_btn)).clicked(actions) {
            if self.current_page < self.total_pages {
                self.current_page += 1;
                self.fetch_error_notebook_from_api();
            }
        }

        if self.button(id!(last_btn)).clicked(actions) {
            if self.current_page < self.total_pages {
                self.current_page = self.total_pages;
                self.fetch_error_notebook_from_api();
            }
        }

        // 处理列表中的按钮点击事件
        for action in actions {
            if let ErrorNotebookAppAction::EditQuestion(id) = action.cast() {
                self.handle_question_action(cx, ErrorNotebookAppAction::EditQuestion(id));
            } else if let ErrorNotebookAppAction::DeleteQuestion(id) = action.cast() {
                self.handle_question_action(cx, ErrorNotebookAppAction::DeleteQuestion(id));
            }else if let ErrorNotebookAppAction::RefreshList(page_result) = action.cast(){
                self.handle_question_action(cx, ErrorNotebookAppAction::RefreshList(page_result));
            }
        }
    }
}

impl Widget for ErrorNotebookApp {
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
                    if item_id < self.answers.len() {
                        let question = &self.answers[item_id];
                        let item_view = list.item(cx, item_id, live_id!(AnswerItem));

                        // 设置 AnswerItem 的 ID
                        if let Some(mut answer_item) = item_view.borrow_mut::<AnswerItem>() {
                            answer_item.question_id = question.question_id.clone();
                            answer_item.id = question.id.clone();
                        }

                        item_view.label(id!(question_id_label)).set_text(cx, &question.question_id);
                        item_view.label(id!(subject_label)).set_text(cx, &question.subject);
                        item_view.label(id!(question_type_label)).set_text(cx, &question.question_type);
                        if &question.correct_result == "正确"{
                            item_view.label(id!(correct_result_label)).set_text(cx, "✅");
                        }else if &question.correct_result == "错误"{
                            item_view.label(id!(correct_result_label)).set_text(cx, "❌");
                        }else{
                            item_view.label(id!(correct_result_label)).set_text(cx, "1/2✅");
                        }
                        // 设置内容（截断过长文本）
                        let k_names = question.k_names.join(",");
                        let content = if k_names.len() > 80 {
                            // 先获取字符迭代器，取前40个字符，然后重新收集为字符串
                            let first_40_chars: String = k_names.chars().take(100).collect();
                            format!("{}...", first_40_chars)
                        } else {
                            k_names.clone()
                        };
                        item_view.label(id!(k_names_label)).set_text(cx, &content);
                        item_view.label(id!(user_answer_label)).set_text(cx, &question.user_answer);
                        item_view.label(id!(remarks_label)).set_text(cx, &question.remarks);
                        item_view.label(id!(create_time_label)).set_text(cx, &question.create_time);
                        item_view.draw_all(cx, scope);
                    }
                }
            }
        }
        DrawStep::done()
    }
}

impl ErrorNotebookApp {
    /// 创建并初始化题目应用并加载示例数据
    pub fn with_sample_data(&mut self) {
        // 添加示例数据
        self.answers = Self::create_sample_answers();
        self.page_size = 5;
        self.total_pages = 5;
        self.current_page = 1;
    }

    /// 创建示例作答数据
    fn create_sample_answers() -> Vec<AnswerRecordVO> {
        vec![
            AnswerRecordVO {
                id: "werewdmlwewqo".to_string(),
                question_id: "Q001".to_string(),
                subject: "数学".to_string(),
                question_type: "选择题".to_string(),
                k_names: vec!["一元二次方程(默认示例数据，请勿操作！)".to_string()],
                create_time: "2025-09-01".to_string(),
                user_answer: "A".to_string(),
                correct_result: "错误".to_string(),
                remarks: "默认示例数据，请勿操作！".to_string(),
                stage: "初中".to_string(),
            }
        ]
    }
    /// 设置题目数据
    pub fn set_answers(&mut self, questions: Vec<AnswerRecordVO>) {
        self.answers = questions;
    }

    /// 应用筛选条件
    fn apply_filter(&mut self, _cx: &mut Cx) {
        self.fetch_error_notebook_from_api();
    }

    /// 重置筛选条件
    fn reset_filter(&mut self, cx: &mut Cx) {
        self.filter = LearnRecordSearch::default();
        self.current_page = 1;
        // 重置输入框
        self.text_input(id!(k_name_input)).set_text(cx, "");
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

    /// 通过Action方式更新题目列表 - 这是主要的渲染接口
    pub fn handle_question_action(&mut self, cx: &mut Cx, action: ErrorNotebookAppAction) {
        match action {
            ErrorNotebookAppAction::RefreshList(page_result) => {
                self.set_answers(page_result.data);
                self.update_pagination(cx,page_result.page_num,page_result.page_size,page_result.total_count);
            },
            ErrorNotebookAppAction::FilterChanged => {
                self.apply_filter(cx);
            },
            ErrorNotebookAppAction::EditQuestion(id) => {
                println!("编辑题目: {}", id);
                // 发送编辑事件给上级组件
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    ErrorNotebookAppAction::EditQuestion(id)
                );
            },
            ErrorNotebookAppAction::DeleteQuestion(id) => {
                println!("删除题目: {}", id);
                // 实现删除逻辑
                self.answers.retain(|q| q.question_id != id);
                self.apply_filter(cx);

                // 发送删除事件给上级组件
                cx.widget_action(
                    self.widget_uid(),
                    &Default::default(),
                    ErrorNotebookAppAction::DeleteQuestion(id)
                );
            },
            _ => {}
        }
    }

    /// 外部调用接口：通过Action方式刷新题目列表
    pub fn refresh_by_action(&mut self, cx: &mut Cx, questions: PageResult<AnswerRecordVO>) {
        self.handle_question_action(cx, ErrorNotebookAppAction::RefreshList(questions));
    }

    /// 外部调用接口：通过Action方式触发筛选
    pub fn filter_by_action(&mut self, cx: &mut Cx) {
        self.handle_question_action(cx, ErrorNotebookAppAction::FilterChanged);
    }
    /// 获取当前分页信息
    pub fn get_pagination_info(&self) -> (usize, usize, usize) {
        (self.current_page, self.total_pages, self.page_size)
    }
}

// 定义题目管理的Action
#[derive(Clone, Debug, DefaultNone)]
pub enum ErrorNotebookAppAction {
    None,
    EditQuestion(String),
    DeleteQuestion(String),
    FilterChanged,
    RefreshList(PageResult<AnswerRecordVO>),
}

// 为外部模块提供便利的初始化方法
impl ErrorNotebookApp {
    /// 初始化题目应用并加载示例数据
    /// 这是外部使用的主要入口点
    pub fn init_with_sample_data(&mut self) {
        self.with_sample_data();
    }

    fn fetch_error_notebook_from_api(&self) {
        let page_num = self.current_page;
        let page_size = self.page_size;
        let params = self.filter.clone();
        tokio::spawn(async move {
            error!("正在从外部API获取数据...");
            let client = AnswerClient::new(format!("{}/{}",SERVER_HOST,"api/error_notebook/search"));
            let req = PageRequest{
                page_num,
                page_size,
                params: Some(params),
            };
            let response = client.search(req).await;
            match response {
                Ok(result) => {
                    error!("检索返回:{:?}",result);
                    Cx::post_action(ErrorNotebookAppAction::RefreshList(result))
                }
                Err(err) => {
                    Cx::post_action(ActionNotificationPopupAction::Fail(format!("检索失败:{err}")))
                }
            }
        });
    }

    /// 获取当前显示的题目列表
    pub fn get_current_questions(&self) -> &Vec<AnswerRecordVO> {
        &self.answers
    }
}

