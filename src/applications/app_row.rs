use crate::applications::AppState;
use makepad_widgets::*;

live_design! {
    use link::widgets::*;
    use crate::applications::app_card::AppCard;

    pub AppRow = {{AppRow}} {
        width: Fill, height: Fit
        flow: Right
        spacing: 10
        margin: {bottom: 10}

        <PortalList> {
            width: Fill
            flow: Down
            spacing: 5
            AppCard = <AppCard> {}
        }
	}
}

#[derive(Live, LiveHook, Widget)]
pub struct AppRow {
    #[deref]
    view: View,
}

impl Widget for AppRow {
    fn draw_walk(
        &mut self,
        cx: &mut Cx2d,
        scope: &mut Scope,
        walk: Walk,
    ) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<AppState>().unwrap();
                let row_idx = *scope.props.get::<usize>().unwrap();

                list.set_item_range(cx, 0, state.num_apps_for_row(row_idx));
                while let Some(item_idx) = list.next_visible_item(cx) {
                    if item_idx >= state.num_apps_for_row(row_idx) {
                        continue;
                    }

                    let item_widget_id = live_id!(AppCard);
                    let item = list.item(cx, item_idx, item_widget_id);

                    let absolute_image_idx = state.first_app_idx_for_row(row_idx) + item_idx;

                    item.apply_over(cx, live!{ image_index: (absolute_image_idx) });

                    let filtered_image_idx = state.filtered_app_ides[absolute_image_idx];
                    // 渲染app
                    let app_info = &state.apps[filtered_image_idx];

                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
