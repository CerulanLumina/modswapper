use eframe::egui::{CentralPanel, Context, ScrollArea};
use eframe::{App, AppCreator, Frame};

mod viewmodel;
mod widgets;

mod filterlogic;
#[cfg(feature = "devdemo")]
mod viewmodel_demo;

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
        CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut self.filter);
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
