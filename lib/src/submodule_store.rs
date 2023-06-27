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

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

use crate::repo::{Repo};
use crate::settings::UserSettings;
use crate::workspace::{WorkspaceInitError};

pub struct Submodule {
    // TODO make non-pub
    pub repo: Arc<dyn Repo>,
    pub name: String,
}

pub trait SubmoduleStore: Send + Sync + Debug {
    fn name(&self) -> &str;
    // TODO This is a hack to avoid a bigger refactor of the git clone code. In
    // the long run, we probably don't want to expose such low level details - a
    // better alternative would be a method that clones the submodule into the
    // store.
    //
    // Given the name of a submodule, return the path that it should be cloned
    // to (for consumption by the `jj git clone` machinery).
    fn get_submodule_path(&self, submodule: &str) -> PathBuf;
    fn load_submodule(
        &self,
        user_settings: &UserSettings,
        submodule: &str,
    ) -> Result<Option<Submodule>, WorkspaceInitError>;
}
