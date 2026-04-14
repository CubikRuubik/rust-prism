# Upstream Sync from go-prism

## Changes from go-prism PR #26: added allowed-repos

### Features
- Added `allowed-repos` configuration to the sync-changes workflow, restricting cross-repo PR creation to explicitly listed repositories
- Added `CubikRuubik/rust-prism` as an allowed repository for the `create_pull_request` safe output

### Configuration
- Updated `sync-changes.md` workflow definition to include `allowed-repos` list under `create-pull-request` safe outputs
- Updated `sync-changes.lock.yml` compiled workflow with new safe outputs config including `allowed_repos` field
- Added `repo_params` for `create_pull_request` to expose and document the `target-repo` parameter

### Internal
- Updated workflow frontmatter hash and prompt delimiter tokens to reflect updated configuration
