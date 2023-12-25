use eframe::egui::{CentralPanel, Context, ScrollArea, ViewportBuilder, ViewportId};
use eframe::{App, AppCreator, Frame};

mod viewmodel;
mod widgets;

mod filterlogic;
mod rfd_worker;
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

        let open = { self.new_swap_set_window.read().unwrap().open };
        if open {
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("New Swap Set"),
                ViewportBuilder::default()
                    .with_title("New Swap Set")
                    .with_inner_size([370.0, 200.0]),
                windows::new_swap_set::new_swap_set_window(self.new_swap_set_window.clone()),
            );
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(&mut self.filter);
                ui.separator();
                let open = { self.new_swap_set_window.read().unwrap().open };
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

#[cfg(feature = "devdemo")]
fn create_demo_viewmodel() -> viewmodel::MainWindowViewModel {
    viewmodel_demo::generate_view_model()
}
