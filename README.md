# ff-open

Open links from CLI tools in a per-folder Firefox profile on macOS.

CLI tools that open a browser (login flows like `az login`, `gh auth login`,
`npx @auth0/auth0-mcp-server init`) use the system default browser. `ff-open`
routes them to the Firefox profile for the folder you're working in, e.g. work
repos to a work profile and personal repos to a personal one.

## How it works

`ff-open` builds a single binary named `open`. A folder's mise config sets
`FIREFOX_PROFILE` and wires the binary in two ways:

- **`BROWSER`**: tools that honour `$BROWSER` (Python `webbrowser`, `gh`, git)
  call it directly.
- **`PATH`**: tools that shell out to `open <url>` (the npm `open` package,
  Go's `pkg/browser`, shell scripts) find it ahead of `/usr/bin/open`.

When every argument is an `http://` or `https://` URL and `FIREFOX_PROFILE` is
set, it runs:

```
/Applications/Firefox.app/Contents/MacOS/firefox -P "$FIREFOX_PROFILE" --new-tab <urls>
```

If that profile is already running, Firefox opens the tab in it; otherwise it
starts the profile. Anything else (`open .`, `open file.pdf`, `open -a Slack`,
any flags) is passed unchanged to `/usr/bin/open`.

### Not covered

- Tools that call `/usr/bin/open` by absolute path.
- Native apps that open URLs via LaunchServices (`NSWorkspace`).
- GUI apps launched from the Dock, which don't get mise's environment.

## Install

Requires Rust (cargo) and [mise](https://mise.jdx.dev) activated in your shell.

```sh
cargo install --path . --root ~/.local/share/ff-open
```

Install to its own root, not `~/.cargo/bin`: that is usually on `PATH`
globally, which would shadow `open` everywhere.

Then add to the `[env]` section of each folder's `mise.toml` (see
[examples/mise.toml](examples/mise.toml)):

```toml
[env]
BROWSER = "{{env.HOME}}/.local/share/ff-open/bin/open"
_.path = ["{{env.HOME}}/.local/share/ff-open/bin"]
FIREFOX_PROFILE = "Work"
```

New config files need `mise trust`. The setting applies to the folder and
everything beneath it.

## Firefox profiles

`FIREFOX_PROFILE` is a profile `Name` from
`~/Library/Application Support/Firefox/profiles.ini`:

```sh
grep '^Name=' ~/Library/Application\ Support/Firefox/profiles.ini
```

## Check it

From a configured folder:

```sh
command -v open                          # ~/.local/share/ff-open/bin/open
open https://example.com                 # opens in the folder's profile
python3 -m webbrowser https://example.com
```

Outside configured folders `command -v open` is `/usr/bin/open`.

## Uninstall

Remove the `BROWSER`, `_.path` and `FIREFOX_PROFILE` lines from your mise
configs, then `rm -rf ~/.local/share/ff-open`.
