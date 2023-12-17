use crate::ui::viewmodel::Filter;
use eframe::egui::{Response, TextEdit, Ui, Widget};

impl Widget for &mut Filter {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add(TextEdit::singleline(&mut self.filter).hint_text("Filter"))
    }
}
