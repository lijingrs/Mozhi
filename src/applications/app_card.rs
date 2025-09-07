use makepad_widgets::*;
live_design! {
    use link::widgets::*;
    use crate::shared::styles::*;
    pub AppCard = {{AppCard}}<RoundedShadowView> {
        width: Fit, height: 160
        margin: {left: 5, right: 5, top: 10, bottom: 10}
        padding: {left: 20, right: 20, top: 20, bottom: 20}
        cursor: Hand
        show_bg: true
        draw_bg: {
            color: #fff
            border_radius: 12,
            uniform shadow_color: #0001
            shadow_radius: 6.0,
            shadow_offset: vec2(0.0, 2.0)
        }

        content = <View> {
            flow: Down
            spacing: 15

            header = <View> {
                flow: Right
                spacing: 15
                align: {x: 0.0, y: 0.5}

                app_icon = <View> {
                    width: 50, height: 50

                    icon_image = <Image> {
                        width: 50, height: 50
                        visible: false
                    }

                    icon_label_wrapper = <RoundedView> {
                        width: 50, height: 50
                        show_bg: true
                        draw_bg: {
                            color: #4F46E5
                            border_radius: 25
                        }
                        align: {x: 0.5, y: 0.5}

                        icon_label = <Label> {
                            draw_text: {
                                text_style: <BOLD_FONT>{font_size: 20}
                                color: #fff
                            }
                        }
                    }
                }
                app_info = <View> {
                    flow: Down
                    spacing: 5
                    app_name = <Label> {
                        draw_text: {
                            text_style: <BOLD_FONT>{font_size: 16}
                            color: #1f2937
                        }
                    }
                    app_status = <Label> {
                        text: "可用"
                        draw_text: {
                            text_style: <REGULAR_FONT>{font_size: 12}
                            color: #10b981
                        }
                    }
                }
            }

            app_description = <Label> {
                width: Fill
                draw_text: {
                    text_style: <REGULAR_FONT>{font_size: 13}
                    color: #6b7280
                    wrap: Word
                }
            }
        }
    }
}
#[derive(Widget, LiveHook, Live)]
pub struct AppCard {
    #[deref]
    view: View,
    #[live]
    app_index: usize,
    #[live]
    app_id: String,
}
impl Widget for AppCard {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        match event.hits(cx, self.view.area()) {
            Hit::FingerUp(_) => {
                cx.action(AppCardClickedAction::Clicked(self.app_id.clone()));
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
#[derive(Clone, Debug, DefaultNone)]
pub enum AppCardClickedAction{
    None,
    Clicked(String),
}