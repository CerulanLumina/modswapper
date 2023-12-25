use eframe::egui::{Align, CollapsingHeader, Grid, Layout, Response, TextEdit, TextStyle, Ui};
use uuid::Uuid;

use crate::ui::viewmodel::{ProfileListViewModel, ProfileViewModel};

pub struct ProfileTable<'a> {
    label: &'a mut String,
    uuid: &'a Uuid,
    source_directories: Option<&'a [String]>,
    target_directories: Option<&'a mut [String]>,
    open: Option<bool>,
}

impl<'a> ProfileTable<'a> {
    pub fn new(label: &'a mut String, uuid: &'a Uuid) -> ProfileTable<'a> {
        ProfileTable {
            label,
            uuid,
            source_directories: None,
            target_directories: None,
            open: None,
        }
    }

    pub fn with_source_directories(self, source_directories: &'a [String]) -> Self {
        Self {
            source_directories: Some(source_directories),
            ..self
        }
    }

    pub fn with_target_directories(self, target_directories: &'a mut [String]) -> Self {
        Self {
            target_directories: Some(target_directories),
            ..self
        }
    }

    pub fn open(self, open: Option<bool>) -> Self {
        Self { open, ..self }
    }

    pub fn show(self, ui: &mut Ui) {
        let target_directories = self
            .target_directories
            .expect("Did not initialize target_directories for ProfileTable");
        let source_directories = self
            .source_directories
            .expect("Did not initialize source_directories for ProfileTable");
        CollapsingHeader::new(self.label.as_str())
            .id_source(self.uuid)
            .default_open(self.open.unwrap_or(false))
            .show(ui, |ui| {
                // TODO: Use multifilelist
                Grid::new(self.uuid).num_columns(2).show(ui, |grid| {
                    for (source, target) in
                        source_directories.iter().zip(target_directories.iter_mut())
                    {
                        grid.code(source);
                        TextEdit::singleline(target)
                            .font(TextStyle::Monospace)
                            .show(grid);
                        grid.end_row();
                    }
                })
            })
            .header_response
            .context_menu(|ui| {
                ui.text_edit_singleline(self.label);
            });
    }
}

pub struct ProfileSelectorWidget<'a> {
    selected: Option<bool>,
    profile_view_model: &'a mut ProfileViewModel,
    source_directories: Option<&'a [String]>,
}

impl<'a> ProfileSelectorWidget<'a> {
    pub fn from_profile_view_model(profile_view_model: &'a mut ProfileViewModel) -> Self {
        Self {
            profile_view_model,
            selected: None,
            source_directories: None,
        }
    }

    pub fn with_selected(self, selected: bool) -> Self {
        Self {
            selected: Some(selected),
            ..self
        }
    }

    pub fn with_source_directories(self, source_directories: &'a [String]) -> Self {
        Self {
            source_directories: Some(source_directories),
            ..self
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let source_directories = self
            .source_directories
            .expect("Did not initialize source_directories for ProfileWidget");
        let res = ui
            .with_layout(Layout::left_to_right(Align::Min), |ui| {
                let res = ui
                    .add_enabled_ui(!self.selected.unwrap_or(false), |ui| ui.button("Swap"))
                    .inner;
                ProfileTable::new(
                    &mut self.profile_view_model.label,
                    &self.profile_view_model.uuid,
                )
                .with_source_directories(source_directories)
                .with_target_directories(&mut self.profile_view_model.target_directories)
                .open(self.selected)
                .show(ui);
                res
            })
            .inner;
        res
    }
}

pub struct ProfileListWidget<'a> {
    profile_list_view_model: &'a mut ProfileListViewModel,
    selected: Option<&'a Uuid>,
    source_directories: Option<&'a [String]>,
}

impl<'a> ProfileListWidget<'a> {
    pub fn from_profile_list(profile_list_view_model: &'a mut ProfileListViewModel) -> Self {
        Self {
            profile_list_view_model,
            selected: None,
            source_directories: None,
        }
    }

    pub fn with_source_directories(self, source_directories: &'a [String]) -> Self {
        Self {
            source_directories: Some(source_directories),
            ..self
        }
    }

    pub fn with_selected(self, selected: Option<&'a Uuid>) -> Self {
        Self { selected, ..self }
    }

    pub fn show(self, ui: &mut Ui) -> ProfileResponse {
        let source_directories = self
            .source_directories
            .expect("Did not initialize source_directories for ProfileListWidget");
        let selected = self.selected;
        self.profile_list_view_model
            .inner
            .iter_mut()
            .map(|(uuid, profile)| {
                let selected = selected.map(|selected| selected == uuid).unwrap_or(false);
                let profile = ProfileSelectorWidget::from_profile_view_model(profile)
                    .with_source_directories(source_directories)
                    .with_selected(selected);
                (uuid, profile.show(ui))
            })
            .find(|(_, res)| res.clicked())
            .map(|(uuid, _)| ProfileResponse::ProfileSwitch(*uuid))
            .unwrap_or(ProfileResponse::None)
    }
}

pub enum ProfileResponse {
    None,
    ProfileSwitch(Uuid),
}
