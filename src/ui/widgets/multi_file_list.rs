use crate::rfd_service::RFD_INVOKER;
use eframe::egui::{Align, Layout, TextEdit, TextStyle, Ui};
use uuid::Uuid;

pub struct MultiFileList<'a, I>
where
    I: Iterator<Item = &'a mut String>,
{
    id_source: Uuid,
    file_list: I,
}

impl<'a, I: Iterator<Item = &'a mut String>> MultiFileList<'a, I> {
    pub fn new(file_list: I, id_source: Uuid) -> Self {
        Self {
            id_source,
            file_list,
        }
    }

    fn show_maybe_additional(self, ui: &mut Ui, opts: Option<(Align, impl FnMut(usize, &mut Ui))>) {
        let (align, mut add_contents) =
            opts.map(|a| (Some(a.0), Some(a.1))).unwrap_or((None, None));
        let mut changed = RFD_INVOKER.poll(&self.id_source);
        for (idx, source) in self.file_list.enumerate() {
            if let Some((path, _)) = maybe_extract(&mut changed, idx) {
                *source = path;
            }
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

fn maybe_extract(opt: &mut Option<(String, usize)>, idx: usize) -> Option<(String, usize)> {
    let extract = { opt.is_some() && opt.as_ref().unwrap().1 == idx };
    if extract {
        opt.take()
    } else {
        None
    }
}
