use std::collections::HashMap;

use uuid::Uuid;

pub(super) struct MainWindowViewModel {
    pub filter: Filter,
    #[cfg(feature = "ui-add-edit")]
    pub new_swap_set_window: NewSwapSetWindow,
    #[cfg(feature = "ui-add-edit")]
    pub new_profile_window: NewProfileWindow,
    pub swap_set_list: SwapSetListViewModel,
}

pub(super) struct Filter {
    pub filter: String,
}

#[cfg(feature = "ui-add-edit")]
pub(super) struct NewSwapSetWindow {
    pub label: String,
    pub source_directories: Vec<String>,
}

#[cfg(feature = "ui-add-edit")]
pub(super) struct NewProfileWindow {
    pub label: String,
    pub target_directories: Vec<String>,
}

pub(super) struct SwapSetListViewModel {
    pub inner: HashMap<Uuid, SwapSetViewModel>,
}

pub(super) struct SwapSetViewModel {
    pub profiles: ProfileListViewModel,
    pub label: String,
    pub uuid: Uuid,
    pub current_profile: Option<Uuid>,
    pub source_directories: Vec<String>,
}

pub(super) struct ProfileListViewModel {
    pub inner: HashMap<Uuid, ProfileViewModel>,
}

pub(super) struct ProfileViewModel {
    pub label: String,
    pub uuid: Uuid,
    pub target_directories: Vec<String>,
}
