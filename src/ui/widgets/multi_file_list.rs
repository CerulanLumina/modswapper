use crate::rfd_service::RFD_INVOKER;
use eframe::egui::scroll_area::ScrollBarVisibility;
use eframe::egui::{
    Align, Grid, Label, Layout, RichText, ScrollArea, Sense, Ui, Widget, WidgetText,
};
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
        Grid::new(self.id_source)
            .num_columns(if align.is_some() { 2 } else { 1 })
            .show(ui, |ui| {
                for (idx, source) in self.file_list.enumerate() {
                    if let Some((path, _)) = maybe_extract(&mut changed, idx) {
                        *source = path;
                    }
                    if let Some(Align::Min) = align {
                        add_contents.as_mut().unwrap()(idx, ui);
                    }
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        if ui.button("Browse...").clicked() {
                            RFD_INVOKER.open_dialog(self.id_source, idx);
                        }
                        ScrollArea::horizontal()
                            .id_source((self.id_source, idx))
                            .enable_scrolling(false)
                            .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
                            .show(ui, |ui| {
                                Label::new(WidgetText::RichText(
                                    RichText::new(source.to_owned())
                                        .monospace()
                                        .background_color(ui.style().visuals.extreme_bg_color),
                                ))
                                .sense(Sense::hover())
                                .ui(ui)
                                .on_hover_ui(|ui| {
                                    ui.label(source.to_owned());
                                });
                            });
                    });

                    ui.end_row();
                }
            });
    }

    pub fn show_with_additional(
        self,
        ui: &mut Ui,
        direction: Align,
        add_contents_each: impl FnMut(usize, &mut Ui),
    ) {
        self.show_maybe_additional(ui, Some((direction, add_contents_each)));
    }

    #[allow(dead_code)]
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
