use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

pub struct Model {
    swap_sets: HashMap<Uuid, SwapSet>,
}

pub struct SwapSet {
    pub label: String,
    uuid: Uuid,
    profiles: HashMap<Uuid, Profile>,
    selected_profile: Option<Uuid>,
    source_directories: Vec<String>,
}

pub struct Profile {
    pub label: String,
    uuid: Uuid,
    target_directories: Vec<String>,
}

pub struct ProfileViewMut<'a, I>
where
    I: Iterator<Item = &'a String>,
{
    pub profile: &'a mut Profile,
    pub source_directories: I,
}

impl Model {
    pub fn create_new() -> Model {
        Model {
            swap_sets: HashMap::new(),
        }
    }

    pub fn new_swap_set(&mut self, label: String) -> &mut SwapSet {
        let uuid = Uuid::new_v4();
        self.swap_sets.insert(
            uuid,
            SwapSet {
                uuid,
                label,
                profiles: HashMap::new(),
                selected_profile: None,
                source_directories: Vec::new(),
            },
        );
        self.swap_sets.get_mut(&uuid).unwrap()
    }

    pub fn swap_sets_mut(&mut self) -> impl Iterator<Item = &mut SwapSet> {
        self.swap_sets.values_mut()
    }

    pub fn get_swap_set(&mut self, uuid: &Uuid) -> Option<&mut SwapSet> {
        self.swap_sets.get_mut(uuid)
    }
}

impl SwapSet {
    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn add_source_dir(&mut self, dir: String) {
        self.profiles
            .values_mut()
            .for_each(|profile| profile.target_directories.push(dir.clone()));
        self.source_directories.push(dir);
    }

    pub fn add_profile(&mut self, label: String) -> &mut Profile {
        let uuid = Uuid::new_v4();
        self.profiles.insert(
            uuid,
            Profile {
                uuid,
                label,
                target_directories: self.source_directories.clone(),
            },
        );
        self.profiles.get_mut(&uuid).unwrap()
    }

    pub fn selected_profile(&self) -> Option<&Profile> {
        self.selected_profile
            .as_ref()
            .and_then(|a| self.profiles.get(a))
    }

    pub fn selected_profile_mut(
        &mut self,
    ) -> Option<ProfileViewMut<'_, impl Iterator<Item = &String>>> {
        self.selected_profile
            .as_ref()
            .and_then(|a| self.profiles.get_mut(a))
            .map(|profile| ProfileViewMut {
                profile,
                source_directories: self.source_directories.iter(),
            })
    }

    pub fn select_profile(
        &mut self,
        uuid: &Uuid,
    ) -> Result<ProfileViewMut<'_, impl Iterator<Item = &String>>, ModelError> {
        if !self.profiles.contains_key(uuid) {
            return Err(ModelError::SelectedProfileOutOfBounds { requested: *uuid });
        }
        self.selected_profile = Some(*uuid);
        Ok(self.selected_profile_mut().unwrap())
    }

    pub fn source_directories(&self) -> impl Iterator<Item = &String> {
        self.source_directories.iter()
    }

    pub fn profiles(&self) -> impl Iterator<Item = &Profile> {
        self.profiles.values()
    }

    pub fn profiles_mut(
        &mut self,
    ) -> impl Iterator<Item = ProfileViewMut<'_, impl Iterator<Item = &String>>> {
        self.profiles.values_mut().map(|profile| ProfileViewMut {
            profile,
            source_directories: self.source_directories.iter(),
        })
    }
}

impl Profile {
    pub fn target_directories(&self) -> impl Iterator<Item = &String> {
        self.target_directories.iter()
    }

    pub fn target_directories_mut(&mut self) -> impl Iterator<Item = &mut String> {
        self.target_directories.iter_mut()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

#[derive(Error, Debug)]
pub enum ModelError {
    #[error(
        "The selected profile was out of bounds for the profiles list. Requested: {}",
        requested
    )]
    SelectedProfileOutOfBounds { requested: Uuid },
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
