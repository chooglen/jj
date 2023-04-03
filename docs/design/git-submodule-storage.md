# Git submodule storage

## Objective

Propose a way for submodules to be stored in jj.

## Requirements

- It should be obvious how the design can support these Phase 1 requirements
  will be supported:

  - Submodules can be cloned anew
  - New submodule commits can be fetched
  - Submodule history and branches can be viewed
  - Submodule contents are populated in the working copy

- There shouldn't be any obvious blockers to supporting other requirements in
  the roadmap.


Attention should be paid to the following edge cases:

- How the operation log works across the superproject and submodule during a `jj
  op restore`.
- How the operation log will track submodule commits
- How we can serialize the operation log in a Git-native format*
- How nested submodules will be supported

Additionally, we'd prefer our solution to have the following properties:

### Simplicity: How easy it is to convey the idea; how many moving parts there are

The simpler, the better. Submodules are a potentially complex feature, so we
should aim to use simple ideas to keep things easy for users and developers.

### Effort: How much effort is needed to implement this

The less effort, the better. Shipping something sooner is very useful feedback,
even if we end up abandoning the approach eventually. On the extreme end, even
if a design isn't very promising, it is worth experimenting with if it is low
effort enough.

### Ease of changing direction: How much this affects the rest of jj; how much can be reused between this and other approaches

The easier, the better. Since submodules are highly experimental, we want to
have the option to change direction if needed without burdening the
reviewers/the rest of the codebase too much.

We try to avoid invasive changes unless they benefit jj anyway, or we have very
good reason to believe they will work. An invasive change is almost always high
effort, but the reverse is not necessarily true.

If we have similar approaches that can reuse work, it is less risky to pursue
them.

### (low priority) Extensibility: How well does the approach extend to possible future work

The more extensible, the better. In the long term, we will likely want to extend
this approach to encompass:

- Non-git subrepos
- Colocated Git repos

For now, these are not a priority, but all else being equal, we should prefer
the more extensible approach.

## Alternatives

### Alternative 1: Store Git submodules as Git repos in the main Git backend

Store Git submodules in the Git backend. Since the Git backend contains a Git
repo, a reasonable default would be to keep them in `$GIT_DIR/modules` like Git
does. The Git backend will provide abstractions for jj to access submodules.
Backends that do not support Git submodules will not provide these abstractions.

- Simplicity: Medium
  - Storage format is easy to visualize and explain
  - Operation log will need to be extended to submodules - likely to introduce
    many edge cases, especially around nested submodules.
- Effort: High
  - 'Basic' implementation is easy. The plumbing is purely git, and the
    interface just requires some refactoring of to enable optional capabilities
    on the Git backend,
  - Exending the operation log is a massive change
- Invasiveness: Medium
  - 'Basic' implementation is mostly contained within Git backend
    - TODO(chooglen) Maybe the integration points for the backend don't exist for the working copy. Take a look
  - Operation log changes will be quite disruptive
- Extensibility (Non-git backends): Low
  - Will need to reimplement Git submodules or whatever other 'subrepos' they
    support. May lead to proliferation of features only implemented by single
    backends.
- Extensibility (Colocated git repos): High
  - Literally just a Git superproject

### Alternative 2: Store Git submodules as full jj repos, aka jj subrepos

This would be somewhere in `.jj` but outside of `.jj/store`. There is some
flexibility in how we can express this in code: we could create new abstractions
specifically for submodules, or we could model the subrepos as alternative
backends.

- Simplicity: Medium-High
  - Extremely easy to explain
  - Superproject operation log could track submodule operation or be more
    decoupled (submodules checkouts don't need operation ids)
  - Interface between the superproject and submodule isn't clear yet.
- Effort: Medium-High
  - The logic for jj commands may reside in `jj` cli. To perform them
    recursively, there may be significant refactoring to move functionality from
    `jj` cli into `lib/`.
- Invasiveness: Medium-High
  - Requires plumbing submodule logic through the various code paths that need
    to recurse into submodules (update working copy, fetch, push)
    - TODO(chooglen) Is it just these cases? That's not too bad actually.
- Extensibility (Non-git backends): Medium-High
  - jj repos abstract over the complexities of the backend
  - Not sure how well the storage format works for non-Git subrepos. Perhaps
    other repos have very different lifecyles from Git submodules and we'll end
    up shoehorning Git submodules to try to make it work.
- Extensibility (Colocated git repos): Medium
  - Won't be natively understood by Git, but if the jj subrepo has a Git
    backend, it may not be hard to upstream a change to Git to understand the
    submodule storage.

### Alternative 3: Store Git submodules as alternate Git backends

Teach jj to use different backends and store each Git submodule in its own Git
backend.

- Simplicity: Low-medium
  - Easy to explain in non-nested case
  - Difficult to visualize how nested submodules could be stored - the
    superproject would store a Git backend for the top level submodule, but how
    would the top level submodule see its submodules?
  - Requires multiple operation logs. Coordinating between them seems complex.
- Effort: Medium-High
  - May require significant refactoring to accommodate > 1 backend
- Invasiveness: Medium-High
  - May require significant refactoring to accommodate > 1 backend.
- Extensibility (Non-git backends): Medium-High
  - We can continue to use regular backend abstractions instead of Git-based
    abstractions
  - Might allow some fine-grained control that is harder to do with a full jj
    subrepo
  - Not sure how well the storage format works for non-Git subrepos. Perhaps
    other repos have very different lifecyles from Git submodules and we'll end
    up shoehorning Git submodules to try to make it work.
- Extensibility (Colocated git repos): Medium
  - Won't be natively understood by Git, but upstreaming a change to Git to
    understand the submodule storage should be quite easy
