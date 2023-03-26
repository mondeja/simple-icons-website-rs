# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/) and setup nightly as default.
- Add `wasm-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/): `cargo install --force cargo-make`
- Install NodeJS with npm.
- Install npm dependencies with `npm i`

## Commands

- `cargo make serve`: Serve the website with [Trunk](https://trunkrs.dev/).
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).
- `cargo make`: Run `serve` and `watch-css` in parallel.
- `cargo make tests`: Run the tests with [Playwright](https://playwright.dev/).
- `cargo make formats`: Format files. If you are using VSCode they should be formatted at save.
- `cargo make builds`: Build the website for production.
- `cargo make doc`: Open documentation for components.

### Recommended VSCode extensions

- Install the [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss) extension.
- Install the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.
- Install the [VSBrowser](https://marketplace.visualstudio.com/items?itemName=Phu1237.vs-browser) extension.
