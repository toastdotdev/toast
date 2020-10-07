# Contributing

Thanks for contributing to Toast!

You can find the maintainers and other contributors [in Discord][discord] as well as on the GitHub repo.

## Project setup

The Toast project is a combination of Rust and JavaScript. We use both [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) and [Yarn Workspaces](https://classic.yarnpkg.com/en/docs/workspaces/).

1.  Fork the toast repo on GitHub
1.  `git clone` your fork
1.  Create a branch for your PR with `git checkout -b your-branch-name`
1.  Run `yarn` to bootstrap the yarn workspaces
1.  Run `cargo build` to build the Rust workspaces

### Project Layout

There are a few files to be aware of.

- The `toast` directory is the Rust code for the Toast binaries.
- `svgrs` is unfinished tooling for SVG optimization and conversion to JSX.
- `toast-node-wrapper` is the package that turns into `bread` or `toast` on NPM
- `Cargo.*` files are Rust related, while `yarn.lock` is yarn/node related

### Node setup

- You probably want [nvm](https://github.com/nvm-sh/nvm)
- Run `nvm i v14` if you don't have the latest v14 of node installed

### Rust setup

If you are new to Rust, you can learn the language by [going through Rustlings with these videos](https://egghead.io/playlists/learning-rust-by-solving-the-rustlings-exercises-a722).

- You'll want to install [rustup](https://rustup.rs/)
- after rustup is installed, you'll need a nightly toolchain to work with Toast so run `rustup toolchain install nightly`
- You can test your install with the following commands, which should both work

```bash
rustc --version
cargo --version
```

## What counts as a contribution?

There are plenty of [open issues][issues] that may fit your skills and expertise. We also highly value documentation changes, user feedback on issues, and more. Code commits are not the only way to contribute. You may also wish to check out the [www.toast.dev issues](https://github.com/toastdotdev/toast/issues).

[issues]: https://github.com/toastdotdev/toast/issues
[www-issues]: https://github.com/toastdotdev/www.toast.dev/issues
[rust]: https://www.rust-lang.org/learn/get-started
[discord]: https://discord.gg/m2RdVRA
