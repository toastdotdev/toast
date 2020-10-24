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

## Building the Rust binary
While making changes to the Rust files, it's helpful to build the project as you go. You can do that by running the following command at the root of the repo: 

```bash
cargo build
```
## Running the binary locally
It can be useful to test out your changes locally by building a Toast site. However, the way Toast works, you can't just build and run the binary directly. Instead, you'll need to put your binary where the Node wrapper expects the binary to be. If you're on an operating system that supports Make, you can use Make to build and copy the binary to the correct directory. Otherwise, you'll need to copy the file manually. 
**Note** that all installs of Toast look for the binary in the same location. You'll want to replace the binary if you don't want to use your local build when working on your own Toast sites. 

### Using Make to install the binary
If you can use Make, you can build the debug version of the binary (same type as cargo build) and copy it to the appropriate destination by running 
```bash
make build-debug-install
```
If you need to test the performance of your changes, you can instead install the production binary by running 
```bash
make build-production-install
```
### Installing the binary manually
If you can't use Make, you can manually install the binary. First, build the binary by running 
```bash
cargo build
# if you need the production build, add --release to the end
```
This will build the binary out to either `target/debug/toast` (if you didn't pass the `--release` flag) or `target/rls/toast` (if you did). 

Then, you need to find out where the binary needs to be placed for Toast to run it. To do that, you can run 

```bash
yarn workspace toast printBinaryPath
```
This will print the global path to the Toast binary. You can then copy your binary from the above path to that location. For a more streamlined process, you could run one of the following sets of commands: 

```bash
# bash
# get directory to install binary to
installDir=$(node ./toast-node-wrapper/binary-management/printInstallDirectory.js) 
# if the install directory doesn't exist, make it
[ -d $installDir ] || mkdir -p $installDir
# copy the binary to the appropriate place
cp target/debug/toast $(node './toast-node-wrapper/binary-management/printBinaryPath')
```
```powershell
# powershell
# if install directory doesn't exist, make it
if ( -not (Test-Path $(node '.\toast-node-wrapper\binary-management\printInstallDirectory.js'))) {mkdir node '.\toast-node-wrapper\binary-management\printInstallDirectory.js'}
Copy-Item 'target\debug\toast.exe' $(node .\toast-node-wrapper\binary-management\printBinaryPath)
```
Once you've installed the binary, you should be able to run Toast. You can build the `test-toast-site` by running 
`yarn workspace test-toast-site toast incremental . ` This site is pretty bare-bones, and only has one page to generate. But it can at least tell you that your changes are working.  


### Reverting to using a released binary
When you've finished working on and testing your local binary, you will probably want to revert to using a release binary. To do that, you'll want to delete the binary from the path printed when you run `yarn workspace toast printBinaryPath`. You'll then want to delete your node_modules and reinstall them. This should replace

## What counts as a contribution?

There are plenty of [open issues][issues] that may fit your skills and expertise. We also highly value documentation changes, user feedback on issues, and more. Code commits are not the only way to contribute. You may also wish to check out the [www.toast.dev issues](https://github.com/toastdotdev/www.toast.dev/issues).

[issues]: https://github.com/toastdotdev/toast/issues
[www-issues]: https://github.com/toastdotdev/www.toast.dev/issues
[rust]: https://www.rust-lang.org/learn/get-started
[discord]: https://discord.gg/m2RdVRA
