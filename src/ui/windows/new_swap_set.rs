use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

use eframe::egui::{
    CentralPanel, Context, ScrollArea, TextEdit, ViewportCommand,
};

use crate::ui::viewmodel::{NewSwapSetWindow, NewSwapSetWindowState};
use crate::ui::widgets::MultiFileList;

pub fn new_swap_set_window(
    state: Arc<RwLock<NewSwapSetWindow>>,
) -> impl Fn(&Context, eframe::egui::ViewportClass) + Send + Sync + 'static {
    move |ctx, _viewport_class| {
        ctx.input(|input| {
            if input.viewport().close_requested() {
                state.write().unwrap().open = false;
            }
        });
        let inner_arc = { state.read().unwrap().inner.clone() };
        let rfd = { state.read().unwrap().rfd.clone() };
        let mut inner_guard = inner_arc.lock().unwrap();
        let inner = &mut *inner_guard;

        if let Some(idx) = inner.file_dialog_index {
            if let Some(path_opt) = rfd.latest_file_picked() {
                inner.file_dialog_index = None;
                if let Some(path) = path_opt {
                    *inner.source_directories.get_mut(idx).unwrap() = path;
                }
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Label: ");
                        ui.centered_and_justified(|ui| {
                            TextEdit::singleline(&mut inner.label)
                                .hint_text("My new swap set")
                                .show(ui);
                        });
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Source Directories");
                        ui.separator();
                        if ui.button("Add Source Directory...").clicked() {
                            inner.source_directories.push(String::new());
                        }
                    });
                    MultiFileList::new(&mut inner.source_directories, &mut inner.file_dialog_index, &rfd)
                        .show(ui);
                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        if ui.button("Create").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                            state.write().unwrap().open = false;
                        }
                    });
                });
        });
    }
}
