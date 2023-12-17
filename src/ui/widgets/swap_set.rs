use eframe::egui::{CollapsingHeader, CollapsingResponse, RichText, Ui, WidgetText};

use crate::ui::viewmodel::{Filter, SwapSetListViewModel, SwapSetViewModel};
use crate::ui::widgets::profile::{ProfileListWidget, ProfileResponse};

impl SwapSetViewModel {
    pub fn show(
        &mut self,
        _append_label: Option<impl Into<WidgetText>>,
        ui: &mut Ui,
    ) -> CollapsingResponse<()> {
        let res = CollapsingHeader::new(&self.label)
            .id_source(self.uuid)
            .show(ui, |ui| {
                if let Some(current_profile) = &self.current_profile {
                    ui.horizontal(|ui| {
                        ui.label("Current Profile: ");
                        ui.label(
                            RichText::new(&self.profiles.inner.get(current_profile).unwrap().label)
                                .strong(),
                        );
                    });
                }
                ui.separator();
                if let ProfileResponse::ProfileSwitch(uuid) =
                    ProfileListWidget::from_profile_list(&mut self.profiles)
                        .with_selected(self.current_profile.as_ref())
                        .with_source_directories(&self.source_directories)
                        .show(ui)
                {
                    self.current_profile = Some(uuid);
                }
            });
        let header_response = res.header_response.context_menu(|menu| {
            menu.text_edit_singleline(&mut self.label);
        });
        CollapsingResponse {
            header_response,
            ..res
        }
    }
}

impl SwapSetListViewModel {
    pub fn show(&mut self, filter: &Filter, ui: &mut Ui) -> Vec<CollapsingResponse<()>> {
        filter
            .filter_iter(self.inner.values_mut())
            .map(|swap| swap.show(None::<&str>, ui))
            .collect()
    }
}
