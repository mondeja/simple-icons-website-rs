# Contributing guide

## Setup

- Install Rust with [rustup](https://rustup.rs/).
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/) with `cargo install --force cargo-make`.
- Install [fnm](https://github.com/Schniz/fnm) with `cargo install fnm` and [setup your Shell](https://github.com/Schniz/fnm#shell-setup).
- Install NodeJS with npm and run `npm install`.
- Install Playwright browsers and dependencies with `npx playwright install --with-deps`.
- Create an _.env_ file at the root including a Github personal token with the variable `GITHUB_TOKEN` like `GITHUB_TOKEN=...`.

## Commands

- `cargo make`: Run `serve` and `watch-css` in parallel. Recommended for development.
- `cargo make test`: Build app for production and run tests with [Playwright](https://playwright.dev/).
- `cargo make format`: Format files. If you are using VSCode they should be formatted on save.
- `cargo make lint`: Check formatting of files.
- `cargo make build`: Build the website for production.
- `cargo make serve`: Build the website for production and serve it with [serve](https://www.npmjs.com/package/serve).
- `cargo make watch-css`: Watch the CSS files with [TailwindCSS](https://tailwindcss.com/).

## Testing

Is useful to run only certain tests in a browser. For example: `cargo make test --project=chrome-desktop --grep=header`

## Add translation

- Copy the _app/locales/en-US/_ folder into _app/locales/{id}_ changing `{id}` with the identifier of the translation.

## Arquitecture

### Technologies

- [Leptos](https://docs.rs/leptos) as the components library with a client side rendering approach.
- [TailwindCSS](https://tailwindcss.com/) as the CSS framework.
- [Trunk](https://trunkrs.dev/) as the web server (on development) and application builder (on production).
- [Playwright](https://playwright.dev/) for end to end testing.

### Rust crates

- **_app/_**: Main package with the app entrypoint. It provides the logic where the top level components are composed and handles global states.
- **_components/_**: Components library. It provides the components used in the website.
- **_macros/_**: It provides compile time macros used in the website to statically generate the data provided by the simple-icons npm package.
- **_simple-icons/_**: Simple Icons Rust library. It is a Rust API to the the simple-icons npm package.

### Where to look

- End to end tests are located in _end2end/tests/_. They are written with [Playwright](https://playwright.dev/). Configuration is located at _end2end/playwright.config.ts_.
- The main stylesheet is located at _app/stylesheet.css_ other assets are located at _app/assets/_. Hopefully you don't need to change this style due to the class-based approach of TailwindCSS framework. Configuration is located at _app/tailwind.config.ts_.
- The initial HTML is located at _app/index.html_. It is used by Trunk to generate the distributed HTML. Most frontend assets are located at _app/public/_.

### Compatibility

- Currently tied to Node.js >= 18 to ensure that the `fetch` API is included in the standard library.
- Using the nighly Rust toolchain to ensure some nightly Rust features like trait aliases and async closures for the _components/_ crate.
