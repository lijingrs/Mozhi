use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::shared::resource_imports::*;
    use crate::shared::widgets::MolyButton;
    use crate::landing::shared::*;

    SUCCESS_ICON = dep("crate://self/resources/images/success_icon.png")
    FAILURE_ICON = dep("crate://self/resources/images/failure_icon.png")

    PRIMARY_LINK_FONT_COLOR = #x0E7090
    SECONDARY_LINK_FONT_COLOR = #667085

    PopupDialog = <RoundedView> {
        width: 350
        height: Fit
        margin: {top: 20, right: 20}
        padding: {top: 20, right: 20 bottom: 20 left: 20}
        spacing: 15

        show_bg: true
        draw_bg: {
            color: #fff
            instance border_radius: 4.0
            fn pixel(self) -> vec4 {
                let border_color = #d4;
                let border_size = 1;
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                let body = #fff

                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                )
                sdf.fill_keep(body)

                sdf.stroke(
                    border_color,
                    border_size
                )
                return sdf.result
            }
        }
    }

    PopupCloseButton = <MolyButton> {
        width: Fit,
        height: Fit,

        margin: {top: -8}

        draw_icon: {
            svg_file: (ICON_CLOSE),
            fn get_color(self) -> vec4 {
                return #000;
            }
        }
        icon_walk: {width: 10, height: 10}
    }

    NotificationIcons = <View> {
        width: Fit,
        height: Fit,
        margin: {top: -10, left: -10}
        success_icon = <View> {
            width: Fit,
            height: Fit,
            <Image> {
                source: (SUCCESS_ICON),
                width: 35,
                height: 35,
            }
        }
        failure_icon = <View> {
            visible: false,
            width: Fit,
            height: Fit,
            <Image> {
                source: (FAILURE_ICON),
                width: 35,
                height: 35,
            }
        }
    }

    NotificationContent = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10

        title = <Label> {
            draw_text:{
                text_style: <BOLD_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: "Notification"
        }

        summary = <Label> {
            width: Fill,
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 9},
                word: Wrap,
                color: #000
            }
            text: ""
        }
    }

    pub ActionNotificationPopup = {{ActionNotificationPopup}} {
        width: Fit
        height: Fit

        <PopupDialog> {
            <NotificationIcons> {}
            <NotificationContent> {}
            close_button = <PopupCloseButton> {}
        }
    }

}

#[derive(Clone, Debug, DefaultNone)]
pub enum ActionNotificationPopupAction {
    None,
    CloseButtonClicked,
    Success(String),
    Fail(String),
}

#[derive(Default)]
pub enum ActionResult {
    #[default]
    Success,
    Failure,
}

#[derive(Live, LiveHook, Widget)]
pub struct ActionNotificationPopup {
    #[deref]
    view: View,
    #[layout]
    layout: Layout,
    #[rust]
    result: ActionResult,
    #[rust]
    message: String,
    #[rust]
    timer: Timer,
}

impl Widget for ActionNotificationPopup {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
        if self.timer.is_event(event).is_some() {
            cx.action(ActionNotificationPopupAction::CloseButtonClicked);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self
            .view
            .draw_walk(cx, scope, walk.with_abs_pos(DVec2 { x: 0., y: 0. }));

        DrawStep::done()
    }
}

impl WidgetMatchEvent for ActionNotificationPopup {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        if self.button(id!(close_button)).clicked(actions) {
            cx.action(ActionNotificationPopupAction::CloseButtonClicked);
        }
    }
}

impl ActionNotificationPopup {
    pub fn update_content(&mut self, cx: &mut Cx) {
        match self.result {
            ActionResult::Success => self.show_success_content(cx),
            ActionResult::Failure => self.show_failure_content(cx),
        }
    }

    fn show_success_content(&mut self, cx: &mut Cx) {
        self.view(id!(success_icon)).set_visible(cx, true);
        self.view(id!(failure_icon)).set_visible(cx, false);

        self.view(id!(success_actions)).set_visible(cx, true);
        self.view(id!(failure_actions)).set_visible(cx, false);

        self.label(id!(summary)).set_text(
            cx,
            &self.message
        );
    }

    fn show_failure_content(&mut self, cx: &mut Cx) {
        self.view(id!(success_icon)).set_visible(cx, false);
        self.view(id!(failure_icon)).set_visible(cx, true);

        self.view(id!(success_actions)).set_visible(cx, false);
        self.view(id!(failure_actions)).set_visible(cx, true);

        self.label(id!(summary)).set_text(
            cx,
            &self.message
            );
    }
}

impl ActionNotificationPopupRef {
    pub fn start_auto_close_timer(&mut self, cx: &mut Cx, seconds: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.timer = cx.start_timeout(seconds);
        }
    }
    pub fn cancel_auto_close_timer(&mut self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.timer = Timer::default();
        }
    }
    pub fn set_message(&mut self, cx: &mut Cx, message: &str, result:ActionResult) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.message = message.to_string();
            inner.result = result;
            inner.update_content(cx);
        }
    }
}
