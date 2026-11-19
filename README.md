# Ardoise Editor plugin template

A plugin for [Ardoise Editor](https://github.com/ardoise-editor/ardoise) to
start from: a command that says hello in the status bar, and a panel that
asks your name. It is written in Rust with
[`simpleedit-plugin`](https://github.com/ardoise-editor/plugin-sdk); a
plugin can be written in any language (see the guide).

**The guide:** <https://ardoise-editor.github.io/ardoise/plugins.html>

## Make it yours

1. Click **Use this template** on GitHub (or copy the files).
2. In `plugin.json`, set `id` (`yourname.yourplugin`), `name`, `author` and
   `repository` (`owner/name` of your repository).
3. In `Cargo.toml` and in `.github/workflows/release.yml` (`PROGRAM`),
   rename `hello` to the name of your program; in `plugin.json`, `command`
   is `["bin/<that name>"]`.
4. Write what it does in `src/main.rs`.

## Try it in the editor

```sh
cargo build --release
mkdir -p bin && cp target/release/hello bin/
```

Then in Ardoise Editor: **Preferences › Plugins › Link a folder…** and pick this
folder. The plugin runs from here: build again, and **Uninstall** then link
again (or restart the editor) to load the new build. What it prints to its
standard error is in `plugin.log`, in the plugin's data folder:

- Linux: `~/.local/share/ardoise/plugin-data/<id>/plugin.log`
- macOS: `~/Library/Application Support/ardoise/plugin-data/<id>/plugin.log`

## Publish it

Set the same version in `plugin.json` and `Cargo.toml`, then push a tag:

```sh
git tag v0.1.0 && git push origin v0.1.0
```

The **Release** workflow builds the plugin for Linux (x86_64) and macOS
(Apple Silicon) and publishes `hello-v0.1.0-x86_64-linux.tar.gz`,
`hello-v0.1.0-aarch64-apple-darwin.tar.gz` and `SHA256SUMS`. Anyone can then
install it from **Preferences › Plugins** by typing `owner/name`.

## License

MIT
