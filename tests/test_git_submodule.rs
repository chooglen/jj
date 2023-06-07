// Copyright 2020 The Jujutsu Authors
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

use crate::common::TestEnvironment;

pub mod common;

#[test]
fn test_gitsubmodule_print_gitmodules() {
    let test_env = TestEnvironment::default();
    let workspace_root = test_env.env_root().join("repo");
    git2::Repository::init(&workspace_root).unwrap();
    test_env.jj_cmd_success(&workspace_root, &["init", "--git-repo", "."]);

    std::fs::write(
        workspace_root.join(".gitmodules"),
        "
[submodule \"old\"]
	path = old
	url = https://github.com/old/old.git
",
    )
    .unwrap();

    test_env.jj_cmd_success(&workspace_root, &["new"]);

    std::fs::write(
        workspace_root.join(".gitmodules"),
        "
[submodule \"new\"]
	path = new
	url = https://github.com/new/new.git
",
    )
    .unwrap();

    let stdout = test_env.jj_cmd_success(
        &workspace_root,
        &["git", "submodule", "print-gitmodules", "-r", "@-"],
    );
    insta::assert_snapshot!(stdout, @r###"
	name:old
	url:https://github.com/old/old.git
	path:old
    "###);

    let stdout =
        test_env.jj_cmd_success(&workspace_root, &["git", "submodule", "print-gitmodules"]);
    insta::assert_snapshot!(stdout, @r###"
	name:new
	url:https://github.com/new/new.git
	path:new
    "###);
}

#[test]
fn test_gitsubmodule_clone() {
    let test_env = TestEnvironment::default();
    let super_workspace_root = test_env.env_root().join("repo");
    git2::Repository::init(&super_workspace_root).unwrap();
    test_env.jj_cmd_success(&super_workspace_root, &["init", "--git-repo", "."]);

    std::fs::write(
        super_workspace_root.join(".gitmodules"),
        "
[submodule \"with/slashes/\"]
	path = foo
	url = ../submodule
",
    )
    .unwrap();

    // Set up a repo to clone the submodule from
    let sub_git_path = test_env.env_root().join("submodule");
    git2::Repository::init(&sub_git_path).unwrap();
    let sub_git_repo = git2::Repository::init(sub_git_path).unwrap();
    let signature =
        git2::Signature::new("Some One", "some.one@example.com", &git2::Time::new(0, 0)).unwrap();
    let mut tree_builder = sub_git_repo.treebuilder(None).unwrap();
    let file_oid = sub_git_repo.blob(b"content").unwrap();
    tree_builder
        .insert("file", file_oid, git2::FileMode::Blob.into())
        .unwrap();
    let tree_oid = tree_builder.write().unwrap();
    let tree = sub_git_repo.find_tree(tree_oid).unwrap();
    sub_git_repo
        .commit(
            Some("refs/heads/main"),
            &signature,
            &signature,
            "message",
            &tree,
            &[],
        )
        .unwrap();
    sub_git_repo.set_head("refs/heads/main").unwrap();

    // Actually clone submodule
    let stdout = test_env.jj_cmd_success(
        &super_workspace_root,
        &["git", "submodule", "clone", "with/slashes/"],
    );
    insta::assert_snapshot!(stdout, @r###"
    Cloned submodule "with/slashes/"
    Nothing changed.
    "###);
}
