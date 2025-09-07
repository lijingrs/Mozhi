
pub mod app_card;
mod app_row;
mod app_grid;

pub mod question_app;
pub mod learn_record_app;
pub mod knowledge_graph_app;
pub mod error_notebook_app;

use crate::settings::app_center_screen::AppInfo;
use makepad_widgets::Cx;
pub use question_app::{QuestionApp, QuestionAppAction};

pub fn live_design(cx: &mut Cx) {
    learn_record_app::live_design(cx);
    knowledge_graph_app::live_design(cx);
    error_notebook_app::live_design(cx);
    app_card::live_design(cx);
    question_app::live_design(cx);
}



#[derive(Debug)]
pub struct AppState {
    apps:Vec<AppInfo>,
    filtered_app_ides: Vec<usize>,
    max_apps_per_row: usize,
    current_app_idx: Option<usize>,
    show_alert: bool,
    alert_message: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            apps: AppInfo::get_apps(),
            filtered_app_ides: vec![],
            max_apps_per_row: 3,
            current_app_idx: None,
            show_alert: false,
            alert_message: String::new(),
        }
    }
}

impl AppState {
    pub(crate) fn num_apps(&self) -> usize {
        self.filtered_app_ides.len()
    }

    pub(crate) fn num_rows(&self) -> usize {
        self.num_apps().div_ceil(self.max_apps_per_row)
    }

    pub(crate) fn first_app_idx_for_row(&self, row_idx: usize) -> usize {
        row_idx * self.max_apps_per_row
    }

    pub(crate) fn num_apps_for_row(&self, row_idx: usize) -> usize {
        let first_image_idx = self.first_app_idx_for_row(row_idx);
        let num_remaining_images = self.num_apps() - first_image_idx;
        self.max_apps_per_row.min(num_remaining_images)
    }
}