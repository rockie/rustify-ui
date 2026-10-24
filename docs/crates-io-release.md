# crates.io release procedure

This procedure releases the nine packages in [the inventory](../release/crates-io.toml).
It does not deploy Pages. Use Python 3.12 and the Rust/Cargo and mbx versions
in [mise.toml](../mise.toml); every build runs through mbx. Publishing is a
separate, explicitly authorized step. Ordinary CI has no registry credential.

The two immutable inputs are library commit **R** and CLI commit **C**.
The release directory **DIR** must persist outside this repository. Keep its
`receipt.json`, `crates/`, `registry/` and verification references together.
The receipt records Cargo, topology, versions, compressed sizes, archive and
Cargo.lock SHA-256, registry checksums, R/C, template lock and upload state.
Never put credentials in this directory, a command argument or a commit.

## Prepare and freeze

1. Finish the milestone's local tests, format, CSS and Clippy gates. Commit
   the library candidate R, including the CLI implementation and candidate
   package documentation. Leave the unknown library revision unset in the
   initial CLI record. Do not write R into R itself.
2. Run the plan's library/non-CLI regression gates from a clean detached R.
   Run the preparation commands from the current release-tooling checkout;
   the script creates its own clean R/C worktrees. Keep source browser fixtures
   on that checkout's committed locks. Then assemble libraries:

   ```sh
   mbx fetch --locked
   mbx fetch --locked --manifest-path site/Cargo.toml
   mbx fetch --locked --manifest-path tests/docs-ssg/fixtures/components/Cargo.toml
   mbx fetch --locked --manifest-path tests/dom-only/Cargo.toml
   mbx fetch --locked --manifest-path tests/dom-only/gpu-negative/Cargo.toml
   mbx fetch --locked --manifest-path tests/dom-only/hydration/Cargo.toml
   mbx fetch --locked --manifest-path makepad/Cargo.toml
   mbx fetch --locked --manifest-path makepad/platform/script/Cargo.toml
   python3.12 scripts/crates-io.py inspect
   python3.12 scripts/crates-io.py package-libraries --rev "$R" --out "$DIR"
   python3.12 scripts/crates-io.py freeze-cli --libraries "$DIR" --library-rev "$R"
   ```

   Fetch all consumer locks: the author projects can pin versions absent from
   the workspace lock. The script takes real locked registry inputs and
   Cargo-generated package index entries. It uses local file source replacement,
   without a registry service. `--no-verify` during assembly is followed by unpacked builds,
   documentation and installation; assembly alone is not a release pass.
3. Review [framework-release.json](../crates/rustify-docs-cli/assets/framework-release.json)
   and [components-Cargo.lock](../crates/rustify-docs-cli/assets/components-Cargo.lock). The lock must contain the actual archive
   checksums. Commit these CLI changes as C; library input bytes must equal R.
   Run the CLI regression gates from C, then:

   ```sh
   python3.12 scripts/crates-io.py package-cli --rev "$C" --libraries "$DIR" --out "$DIR"
   python3.12 scripts/crates-io.py verify --release "$DIR"
   ```

   Verification uses an independent Cargo home, standard release installation
   without `--profile docs-cli`, checks registry identity and unwind recovery,
   builds both templates outside the repository and checks each library in its
   declared docs.rs mode. It repackages R with two real index views, one with an
   extra compatible version, and refuses any archive or lock drift.
4. Complete the planned migration, author simulation and browser matrix using
   that installed archive CLI. Set `RUSTIFY_DOCS_BIN` to the verification's
   binary and `CARGO_HOME` to its staging Cargo home for every child process.
   Record these local paths and results in the milestone record; screenshots
   stay outside package directories. Do not substitute a source-built CLI.

A changed library manifest/source/README/resource or root lock means a new R,
eight new library archives, a new template lock and C. A CLI-only change can
keep R and its eight archives, with a new C and CLI archive. A byte mismatch
with unchanged R is a failure to investigate; do not replace receipt hashes
or silently regenerate the release to hide it.

## Remote preflight

Prepare the commits, hashes and exact commands for user review first. Before
running the following, obtain authorization covering a push of main containing
the workflow/R/C and the two-platform credential-free dispatch:

```sh
git push origin main
gh workflow run crates-io.yml --ref main -f library_rev="$R" -f cli_rev="$C"
gh run list --workflow crates-io.yml --event workflow_dispatch --limit 5
gh run watch "$RUN_ID" --exit-status
```

Confirm the run actually used R/C, read both macOS and Linux conclusions and
retain its run ID/artifacts. This checks portable docs/installation on stable
Rust; it does not establish complete Linux/Windows support or the docs.rs
nightly sandbox result. Do not deploy the official site as part of this release.

## Account and authorization gate

Before any upload, record the user's approval of the nine package names and
versions, actual upload, and later public documentation/tag pushes. Recheck
each name, version and existing owner on crates.io; a name query does not
reserve it. Stop on an unexpected owner or version, rather than selecting a
new name. Confirm the publishing account and verified email.

The publishing person creates a temporary token with `publish-new`, name
patterns covering exactly the inventory (including the classification helper
and style-wasm), and an expiry covering upload/index/docs.rs waits. An existing
crate's new version also needs `publish-update`. Record the account, allowed
names, permissions and expiry, never the token value. The person chooses the
expiry; a short session may outlast its token during new-crate rate limiting.

Choose one credential method:

- Run `cargo login --registry crates-io` interactively and enter the token at
  Cargo's prompt. Never put it after the command or in a shell transcript.
- Inject `CARGO_REGISTRY_TOKEN` from the controlled environment/secret store,
  with Cargo's token credential provider enabled. Never echo or serialize it.

Cargo's [publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish)
and [credential providers](https://doc.rust-lang.org/cargo/reference/registry-authentication.html)
explain account/email and credential storage. The release script refuses source
replacement during production publishing. Remove staging `CARGO_HOME` and
source overrides before this step; keep the verified receipt unchanged.

## Upload and resume

The exact topology comes from the inventory: classification macros → style;
docs-syntax → docs-macros; dom → components → docs → docs-cli, with the
remaining style-wasm package after the public Docs gate. The inventory also
includes every direct internal dependency, rather than just this summary.

```sh
python3.12 scripts/crates-io.py publish --release "$DIR" --group docs
```

For each package the script reads actual registry state, then runs
`mbx publish --registry crates-io --locked -p <name> --dry-run` from clean R
(or C for CLI), compares archive and lock digests, and only then runs the
same publish without `--dry-run`. Each dependency's sparse index and downloaded
bytes must match before continuing. The current conservative cap is 10,000,000
compressed bytes per package; recheck the registry limit at the release gate.
Cargo currently documents a [10 MB limit](https://doc.rust-lang.org/cargo/reference/publishing.html#packaging-a-crate).

- Existing name/version with the same checksum resumes without uploading.
  Different checksum stops; a published version is immutable.
- Timeout or a crash during upload leaves `waiting-index`. Resume queries the
  API/index before any retry. If the outcome remains unknown, investigate the
  registry state; do not reset the receipt to pending just to retry.
- HTTP 429 queries state, then saves `waiting-rate-limit` and `next_retry_at`
  from Retry-After when available. Exit 75 means wait/resume, not rebuild.
  Run the same command after that time. If no retry time could be parsed,
  obtain the publishing person's explicit retry time and pass
  `--retry-at <Unix-seconds>`. Do not immediately loop uploads. The server's
  actual response controls the wait; the plan allows a long new-crate window.
- If credentials expire, replace them through Cargo's normal mechanism and
  resume the same R/C/receipt. Do not repackage or pick a new version merely
  to bypass a wait.

After all eight Docs-group packages are public:

```sh
python3.12 scripts/crates-io.py verify --release "$DIR" --public-docs
python3.12 scripts/crates-io.py publish --release "$DIR" --group remaining
```

The public Docs gate installs from crates.io with a new Cargo home and root,
without source replacement, and exercises content plus components SSR/hydrate.
The script requires that recorded pass before style-wasm. Partial publication
is retained; there is no cross-package rollback transaction and no automatic yank.

## Public completion and cleanup

Once all nine packages are available, update the repository's public default
installation/upgrade/rollback instructions and CHANGELOG to 0.2.0, commit and
push within the approval scope. Do not change already validated package README
or template bytes after freezing. From public links read back the new wording.

Run `verify --release "$DIR" --public` with another fresh Cargo home/root and
complete the final public author/migration checks. Query all nine exact package
versions and the eight libraries' docs.rs version/build results; wait for all
eight documentation jobs to succeed. A queued or failed library job is an unpassed gate.
The CLI is binary-only, while docs.rs invokes `rustdoc --lib`; retain its
no-library-target build result and verify its public package README, `--help`
and real installation/use instead. The publishing user confirmed this boundary
for 0.2.0 in the milestone record.
Stable preflight is distinct from [docs.rs build configuration](https://docs.rs/about/metadata).
Record actual platforms, sizes, install times, public documentation commit and
remaining limits, without claiming untested Windows or full Linux support.

Only after the final public gate passes, create and push the approved tags:

```sh
git tag rustify-libs-v0.2.0 "$R"
git tag rustify-docs-cli-v0.2.0 "$C"
git push origin refs/tags/rustify-libs-v0.2.0 refs/tags/rustify-docs-cli-v0.2.0
```

For a user rollback, reinstall the old 0.1.32 Git CLI as documented in the
[author guide](docs-ssg.md#install-and-maintain-the-cli) and restore its original
manifest/lock, theme/fonts and complete dist. A future registry rollback pins
a known good CLI version and its matching project files. Yank requires the
publishing person's separate explicit package/version/reason approval; it does
not remove bytes or invalidate existing locks.

At the end, revoke the temporary token on crates.io, then run
`cargo logout --registry crates-io` if it was stored locally and remove the
injected variable from the controlled environment. Logout clears local
credentials; server-side revocation is a separate action. Keep credential-free
receipts and validation records, and link the exact runbook revision executed.
