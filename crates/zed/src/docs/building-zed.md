## Building Zed

## 🚧 TODO 🚧

- [ ] Update prerequisites for open source release.
- [ ] Add details on what permissions the `GITHUB_TOKEN` needs.
- [ ] Update the build process for the open source release of Zed.
- [ ] Remove Vercel/zed.dev steps from the build process.

### Prerequisites

- [zed-industries](https://github.com/zed-industries) GitHub organization access.
- [zed-industries](https://vercel.com/zed-industries) Vercel team access.
- A [GitHub Personal Accesss Token](https://docs.github.com/en/enterprise-server@3.9/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens). Note: A "classic" Personal Access Token is required.

### Dependencies

Ensure the following are installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [GitHub CLI](https://cli.github.com/)
- [Vercel CLI](https://vercel.com/docs/cli)
- [Livekit](https://formulae.brew.sh/formula/livekit)
- [Foreman](https://formulae.brew.sh/formula/foreman)
- [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) and Xcode command line tools (`xcode-select --install`)
- [Postgres](https://postgresapp.com)
- wasm toolchain (`rustup target add wasm32-wasi`)

### Building Zed from Source

Clone the Zed repository:

```bash
gh repo clone zed-industries/zed
```

Clone the Zed.dev repository:

**Note:** The Zed.dev and Zed repositories should be siblings.

```bash
git clone https://github.com/zed-industries/zed.dev
cd zed.dev && npm install
pnpm install -g vercel
```

Link your zed.dev project to Vercel:

```bash
vercel link
    - zed-industries
    - zed.dev
vercel pull
vercel env pull # When you only need to pull the environment variables
```

Open Postgres.app.

From `./path/to/zed/` run `GITHUB_TOKEN={yourGithubAPIToken} script/bootstrap`.

To run the Zed app:

- If you are working on zed:
    - `cargo run`
- If you are just using the latest version, but not working on zed:
    - `cargo run --release`
- If you need to run the collaboration server locally:
    - `script/zed-local`

### Troubleshooting

**`error: failed to run custom build command for gpui v0.1.0 (/Users/path/to/zed)`**:

- Execute: `xcode-select --switch /Applications/Xcode.app/Contents/Developer`

**Homebrew ARM processor error during `script/bootstrap`**:

- Reinstall Homebrew: `/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`
- Add Homebrew to your PATH. Replace `{username}` with your actual username:
  - `echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> /Users/{username}/.zprofile`
  - `eval "$(/opt/homebrew/bin/brew shellenv)"`

**Database seeding issues**:

- Ensure `GITHUB_TOKEN` has correct permissions (`repo` OAuth scope).

### If unstable feature errors arise:

- Clean the build environment and rebuild:
  - `cargo clean`
  - `cargo build`
