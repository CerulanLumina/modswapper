use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use crate::ui::rfd_worker::RFDInvoker;
use uuid::Uuid;

pub(super) struct MainWindowViewModel {
    pub filter: Filter,
    pub new_swap_set_window: Arc<RwLock<NewSwapSetWindow>>,
    // #[cfg(feature = "ui-add-edit")]
    // pub new_profile_window: NewProfileWindow,
    pub swap_set_list: SwapSetListViewModel,
}

pub(super) struct Filter {
    pub filter: String,
}

pub(super) struct NewSwapSetWindow {
    pub inner: Arc<Mutex<NewSwapSetWindowState>>,
    pub open: bool,
    pub rfd: RFDInvoker,
}

impl NewSwapSetWindow {
    pub fn reset(&mut self) {
        let mut v = self.inner.lock().unwrap();
        v.label = String::new();
        v.source_directories = vec![String::new()];
    }
}

pub(super) struct NewSwapSetWindowState {
    pub label: String,
    pub source_directories: Vec<String>,
    pub file_dialog_index: Option<usize>,
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
    // pub rfd: RFDInvoker,
    // pub selected_file_index: Option<usize>,
}
