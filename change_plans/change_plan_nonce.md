# Change Plan: fix(sync-workflow): create change_plans/change_plan_nonce.md to satisfy create-pull-request commit requirement

## Fixes

- **Sync workflow now creates a committed file in dependent repos before opening a PR**: Previously, the sync agent tried to open "description-only" pull requests (with no file changes). This failed because the pull request creation tool requires at least one committed file change on the branch. The workflow now writes a change plan file (`change_plans/change_plan_nonce.md`) to the dependent repository workspace and commits it before creating the pull request, satisfying the commit requirement.

## Changed Behavior

- **New file written per sync**: Each pull request opened in a dependent repository will now include a single committed file: `change_plans/change_plan_nonce.md`, containing the full change description. This is the only file change made to the branch.
- **Guideline updated**: The "No file changes" guideline has been replaced with "Change plan file" — explicitly clarifying that writing this plan file is the intended and only change committed to the sync branch.
- **PR creation includes `path` parameter**: The `create-pull-request` call now includes a `path` parameter pointing to the checked-out dependent repo workspace directory, so the tool knows which local directory to commit and push from.

## Summary

This change fixes a workflow-level bug where description-only PRs (without any committed files) were silently failing. The fix introduces a lightweight, purpose-built file (`change_plans/change_plan_nonce.md`) that carries the change description as its content, giving the PR creation tool the required commit while also providing a human-readable record of why the PR was opened.
