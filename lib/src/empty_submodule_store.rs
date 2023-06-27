// Copyright 2023 The Jujutsu Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(missing_docs)]

use std::path::{Path, PathBuf};

use crate::settings::UserSettings;
use crate::submodule_store::{Submodule, SubmoduleStore};
use crate::workspace::{Workspace, WorkspaceInitError};

#[derive(Debug)]
pub struct EmptySubmoduleStore {}

impl EmptySubmoduleStore {
    pub fn new() -> Self {
        EmptySubmoduleStore{}
    }
}

impl SubmoduleStore for EmptySubmoduleStore {
    fn name(&self) -> &str {
        "empty"
    }

    fn get_submodule_path(&self, _submodule: &str) -> PathBuf {
        PathBuf::new()
    }

    fn load_submodule(
        &self,
        _user_settings: &UserSettings,
        _submodule: &str,
    ) -> Result<Option<Submodule>, WorkspaceInitError> {
        Ok(None)
    }
}
