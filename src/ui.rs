use std::sync::{Arc, RwLock};

use eframe::{App, AppCreator, Frame};
use eframe::egui::{CentralPanel, Context, ScrollArea, ViewportBuilder, ViewportCommand, ViewportId};

use crate::ui::viewmodel::NewSwapSetWindow;

mod viewmodel;
mod widgets;

mod filterlogic;
#[cfg(feature = "devdemo")]
mod viewmodel_demo;
mod windows;

pub fn app() -> AppCreator {
    #[cfg(feature = "devdemo")]
    {
        Box::new(|_cc| Box::new(create_demo_viewmodel()))
    }
    #[cfg(not(feature = "devdemo"))]
    {
        unimplemented!()
    }
}

impl App for viewmodel::MainWindowViewModel {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.set_zoom_factor(1.5f32);

        let open = {
            self.new_swap_set_window.read().unwrap().open
        };
        if open {
            ctx.show_viewport_deferred(ViewportId::from_hash_of("UwU"),
                                       ViewportBuilder::default()
                                           .with_title("New Swap Set"),
                                       new_swap_set_window(self.new_swap_set_window.clone())
            );
        }


        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(&mut self.filter);
                ui.separator();
                let open = {
                    self.new_swap_set_window.read().unwrap().open
                };
                ui.add_enabled_ui(!open, |ui| {
                    if ui.button("New Swap Set").clicked() {
                        let mut win_write = self.new_swap_set_window.write().unwrap();
                        win_write.reset();
                        win_write.open = true;
                    }
                });
            });
            ui.separator();
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.swap_set_list.show(&self.filter, ui);
                });
        });
    }
}

fn new_swap_set_window(state: Arc<RwLock<NewSwapSetWindow>>) -> impl Fn(&Context, eframe::egui::ViewportClass) + Send + Sync + 'static {
    move |ctx, viewport_class| {
        ctx.input(|input| {
            if input.viewport().close_requested() {
                state.write().unwrap().open = false;
            }
        });
        CentralPanel::default().show(ctx, |ui| {
            let inner_arc = {
                state.read().unwrap().inner.clone()
            };
            let mut inner = inner_arc.lock().unwrap();
            ui.horizontal(|ui| {
                ui.label("Label: ");
                ui.text_edit_singleline(&mut inner.label);
            });
            ui.separator();
            ui.label("Source Directories");
            for source in &mut inner.source_directories {
                ui.text_edit_singleline(source);
            }
            if ui.button("Add Source Directory...").clicked() {
                inner.source_directories.push(String::new());
            }
            if ui.button("Create!").clicked() {
                ctx.send_viewport_cmd(ViewportCommand::Close);
                state.write().unwrap().open = false;
            }
        });
    }
}

#[cfg(feature = "devdemo")]
fn create_demo_viewmodel() -> viewmodel::MainWindowViewModel {
    viewmodel_demo::generate_view_model()
}
