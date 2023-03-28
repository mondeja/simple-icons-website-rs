# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/) and setup nightly as default.
- Add `wasm-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/): `cargo install --force cargo-make`
- Install NodeJS with npm and dependencies with `npm install`.

## Commands

- `cargo make serve`: Serve the website with [Trunk](https://trunkrs.dev/).
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).
- `cargo make`: Run `serve` and `watch-css` in parallel. Recommended for development.
- `cargo make tests`: Run the tests with [Playwright](https://playwright.dev/).
- `cargo make formats`: Format files. If you are using VSCode they should be formatted at save.
- `cargo make builds`: Build the website for production.
- `cargo doc --open`: Open documentation. If you want to build the documentation without dependencies, run `cargo doc --open --no-deps`. You can see the packages under `simple-icons-website-*` names.

### Recommended VSCode extensions

- Install the [Tailwind CSS IntelliSense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss) extension.
- Install the [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.
- Install the [VSBrowser](https://marketplace.visualstudio.com/items?itemName=Phu1237.vs-browser) extension.

### Recommended workflow

- Open the project in VSCode with a VSBrowser screen at the side.
- Run `cargo make` in the terminal.
- Enjoy (recommended).

### Arquitecture

#### Technologies

- [Leptos](https://docs.rs/leptos) as the components library with a client side rendering approach.
- [TailwindCSS](https://tailwindcss.com/) as the CSS framework.
- [Trunk](https://trunkrs.dev/) as the web server (on development) and application builder (on production).

#### Rust crates

- `app/`: Main package with the app entrypoint. It provides the logic where the top level components are composed.
- `components/`: Components library. It provides the components used in the website.
- `macros/`: It provides compile time macros used in the website. Used to generate the data provided by the simple-icons npm package.
- `types/`: Common types shared by packages.

#### Where to look

- End to end tests are located in `app/tests/`. They are written with [Playwright](https://playwright.dev/). Configuration is located at `app/playwright.config.js`.
- The main stylesheet and other assets are located at `app/assets/`. Hopefully you don't need to change this style due to the class-based approach of TailwindCSS framework. Configuration is located at `app/tailwind.config.ts`.
- The initial HTML is located at `app/index.html`. It is used by Trunk to generate the final HTML.
