use crate::rfd_service::RFD_INVOKER;
use eframe::egui::{Align, Layout, TextEdit, TextStyle, Ui};
use uuid::Uuid;

pub struct MultiFileList<'a> {
    id_source: Uuid,
    file_list: &'a mut [String],
}

impl<'a> MultiFileList<'a> {
    pub fn new(file_list: &'a mut [String], id_source: Uuid) -> Self {
        Self {
            id_source,
            file_list,
        }
    }

    fn show_maybe_additional(self, ui: &mut Ui, opts: Option<(Align, impl FnMut(usize, &mut Ui))>) {
        let (align, mut add_contents) =
            opts.map(|a| (Some(a.0), Some(a.1))).unwrap_or((None, None));

        for (idx, source) in self.file_list.iter_mut().enumerate() {
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
                        if let Some(Align::Max) = align {
                            add_contents.as_mut().unwrap()(idx, ui);
                        }

                        res
                    })
                    .inner
                })
                .inner;
            if res.clicked() {
                RFD_INVOKER.open_dialog(self.id_source, idx);
            }
        }
        if let Some((path, idx)) = RFD_INVOKER.poll(&self.id_source) {
            if let Some(s) = self.file_list.get_mut(idx) {
                *s = path;
            }
        }
    }

    pub fn show_with_additional(
        self,
        ui: &mut Ui,
        direction: Align,
        add_contents_each: impl FnMut(usize, &mut Ui),
    ) {
        self.show_maybe_additional(ui, Some((direction, add_contents_each)));
    }

    pub fn show(self, ui: &mut Ui) {
        self.show_maybe_additional(ui, None::<(_, fn(usize, &mut Ui))>);
    }
}
