use std::collections::HashMap;
use std::iter;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex, RwLock};

use camino::{Utf8Path, Utf8PathBuf};
use inflector::cases::titlecase::to_title_case;
use names::Name;
use once_cell::sync::Lazy;
use rand::Rng;
use uuid::Uuid;

use crate::ui::viewmodel::*;

const SWAP_SET_COUNT: RangeInclusive<usize> = 30..=50;
const PROFILE_COUNT: RangeInclusive<usize> = 2..=4;

const SOURCE_DIRECTORY_COUNT: RangeInclusive<usize> = 2..=4;

static PROJECT_DIRS: Lazy<Utf8PathBuf> = Lazy::new(|| {
    directories::ProjectDirs::from("xyz", "luminasapphira", "modswapper")
        .expect("Getting modswapper data folder for demo")
        .data_dir()
        .to_owned()
        .try_into()
        .expect("Not UTF-8 paths")
});

static GLOBAL_DIRS: Lazy<Utf8PathBuf> = Lazy::new(|| {
    directories::BaseDirs::new()
        .expect("Getting base dirs")
        .data_local_dir()
        .to_owned()
        .try_into()
        .expect("Not UTF-8 Paths")
});

pub fn generate_view_model() -> MainWindowViewModel {
    MainWindowViewModel {
        filter: Filter {
            filter: "".to_owned(),
        },
        new_swap_set_window: Arc::new(RwLock::new(NewSwapSetWindow {
            inner: Arc::new(Mutex::new(NewSwapSetWindowState {
                label: "".to_string(),
                source_directories: vec![],
            })),
            open: false,
        })),
        // #[cfg(feature = "ui-add-edit")]
        // new_profile_window: NewProfileWindow {
        //     label: "".to_owned(),
        //     target_directories: vec![],
        // },
        swap_set_list: SwapSetListViewModel {
            inner: HashMap::from_iter(
                iter::from_fn(generate_swap_set)
                    .map(|swap| (swap.uuid, swap))
                    .take(rand::thread_rng().gen_range(SWAP_SET_COUNT)),
            ),
        },
    }
}

fn generate_swap_set() -> Option<SwapSetViewModel> {
    let mut names = names::Generator::with_naming(Name::Plain);
    let label = to_title_case(names.next().unwrap().as_str());
    let source_dirs: Vec<String> = names
        .take(rand::thread_rng().gen_range(SOURCE_DIRECTORY_COUNT))
        .map(|a| GLOBAL_DIRS.join(label.as_str()).join(a).into_string())
        .collect();
    let names = names::Generator::with_naming(Name::Plain);
    let profiles = HashMap::from_iter(
        names
            .take(rand::thread_rng().gen_range(PROFILE_COUNT))
            .map(|name| generate_profile(name, source_dirs.as_slice()))
            .map(|pvm| (pvm.uuid, pvm)),
    );

    Some(SwapSetViewModel {
        current_profile: Some(*profiles.keys().next().unwrap()),
        profiles: ProfileListViewModel { inner: profiles },
        label,
        uuid: Uuid::new_v4(),
        source_directories: source_dirs,
    })
}

fn generate_profile(slug: String, source_dirs: &[String]) -> ProfileViewModel {
    ProfileViewModel {
        label: to_title_case(&slug),
        uuid: Uuid::new_v4(),
        target_directories: Vec::from_iter(
            source_dirs
                .iter()
                .map(Utf8Path::new)
                .flat_map(Utf8Path::file_name)
                .map(|source_name| {
                    PROJECT_DIRS
                        .join("instances")
                        .join(&slug)
                        .join(source_name)
                        .into_string()
                }),
        ),
    }
}
