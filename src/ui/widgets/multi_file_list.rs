use eframe::egui::{Align, Direction, Layout, TextEdit, TextStyle, Ui};
use crate::ui::rfd_worker::RFDInvoker;

pub struct MultiFileList<'a> {
    file_list: &'a mut Vec<String>,
    selecting_file_index: &'a mut Option<usize>,
    rfd_invoker: &'a RFDInvoker,
}

impl<'a> MultiFileList<'a> {

    pub fn new(file_list: &'a mut Vec<String>, selecting_file_index: &'a mut Option<usize>, rfd_invoker: &'a RFDInvoker) -> Self {
        Self {
            file_list,
            selecting_file_index,
            rfd_invoker,
        }
    }

    fn show_maybe_additional(self, ui: &mut Ui, opts: Option<(Align, impl FnMut(usize, &mut Ui))>) {
        let mut clicked_idx = None;
        let (align, mut add_contents) = opts.map(|a| (Some(a.0), Some(a.1))).unwrap_or((None, None));

        for (idx, source) in &mut self.file_list.iter_mut().enumerate() {
            let res = ui
                .horizontal(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if let Some(Align::Min) = align {
                            add_contents.as_mut().unwrap()(idx, ui);
                        }
                        let res = ui.button("Browse...");
                        ui.centered_and_justified(|ui| {
                            TextEdit::singleline(source)
                                .font(TextStyle::Monospace)
                                .show(ui);
                        });
                        if let Some(Align::Min) = align {
                            add_contents.as_mut().unwrap()(idx, ui);
                        }

                        res
                    })
                        .inner
                })
                .inner;
            if res.clicked() {
                clicked_idx = Some(idx);
            }
        }
        if self.selecting_file_index.is_none() && clicked_idx.is_some() {
            self.rfd_invoker.open_file_dialog();
            *self.selecting_file_index = clicked_idx;
        }
    }

    pub fn show_with_additional(self, ui: &mut Ui, direction: Align, add_contents_each: impl FnMut(usize, &mut Ui)) {
        self.show_maybe_additional(ui, Some((direction, add_contents_each)));
    }

    pub fn show(self, ui: &mut Ui) {
        self.show_maybe_additional(ui, None::<(_, fn(usize, &mut Ui))>);
    }
}
