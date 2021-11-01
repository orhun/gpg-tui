[![Logo](assets/logo.jpg)](https://github.com/orhun/gpg-tui)

[![GitHub Release](https://img.shields.io/github/v/release/orhun/gpg-tui?style=flat&labelColor=1d1d1d&color=424242&logo=GitHub&logoColor=white)](https://github.com/orhun/gpg-tui/releases)
[![Crate Release](https://img.shields.io/crates/v/gpg-tui?style=flat&labelColor=1d1d1d&color=424242&logo=Rust&logoColor=white)](https://crates.io/crates/gpg-tui/)
[![Coverage](https://img.shields.io/codecov/c/gh/orhun/gpg-tui?style=flat&labelColor=1d1d1d&color=424242&logo=Codecov&logoColor=white)](https://codecov.io/gh/orhun/gpg-tui)
[![Continuous Integration](https://img.shields.io/github/workflow/status/orhun/gpg-tui/Continuous%20Integration/master?style=flat&labelColor=1d1d1d&color=424242&logo=GitHub%20Actions&logoColor=white)](https://github.com/orhun/gpg-tui/actions?query=workflow%3A%22Continuous+Integration%22)
[![Continuous Deployment](https://img.shields.io/github/workflow/status/orhun/gpg-tui/Continuous%20Deployment?style=flat&labelColor=1d1d1d&color=424242&logo=GitHub%20Actions&logoColor=white&label=deploy)](https://github.com/orhun/gpg-tui/actions?query=workflow%3A%22Continuous+Deployment%22)
[![Docker Builds](https://img.shields.io/docker/cloud/build/orhunp/gpg-tui?style=flat&labelColor=1d1d1d&color=424242&label=docker&logo=Docker&logoColor=white)](https://hub.docker.com/r/orhunp/gpg-tui)
[![Documentation](https://img.shields.io/docsrs/gpg-tui?style=flat&labelColor=1d1d1d&color=424242&logo=Rust&logoColor=white)](https://docs.rs/gpg-tui/)

## About

**gpg-tui** is a [Terminal User Interface](https://en.wikipedia.org/wiki/Text-based_user_interface) for [GnuPG](https://gnupg.org/).

It aims to ease the key management operations such as listing/exporting/signing by providing an interface along with the command-line fallback for more complex operations. It is not trying to be a full-fledged interface for _all_ the features that [gpg](https://linux.die.net/man/1/gpg) provides but it tries to bring a more interactive approach to key management.

<details>
  <summary><b>Demo</b></summary>

![](demo/gpg-tui-showcase.gif)

<pre>
<b>gpg-tui</b> --style <span style="color: #036A9BFF"><i>colored</i></span> --splash --homedir <span style="color: #036A9BFF"><i>/etc/pacman.d/gnupg</i></span>
</pre>

</details>

<details>
  <summary>Table of Contents</summary>

- [About](#about)
- [Requirements](#requirements)
- [Installation](#installation)
  - [Cargo](#cargo)
  - [Arch Linux](#arch-linux)
    - [Community](#community)
    - [AUR](#aur)
  - [Gentoo](#gentoo)
  - [Homebrew](#homebrew)
  - [Docker](#docker)
    - [Docker Hub](#docker-hub)
    - [Using the Dockerfile](#using-the-dockerfile)
  - [FreeBSD](#freebsd)
    - [Building from source](#building-from-source)
    - [Binary releases](#binary-releases)
  - [NetBSD](#netbsd)
    - [Install using the package manager](#install-using-the-package-manager)
    - [Building from source](#building-from-source-1)
  - [Manually](#manually)
    - [Building from source](#building-from-source-2)
    - [Binary releases](#binary-releases-1)
- [Usage](#usage)
- [Configuration](#configuration)
- [Key Bindings](#key-bindings)
  - [User Interface](#user-interface)
  - [Key Management](#key-management)
- [Approach](#approach)
  - [Detail Levels](#detail-levels)
  - [Key Information](#key-information)
  - [User Information](#user-information)
- [Features](#features)
  - [User Interface](#user-interface-1)
    - [Scrolling](#scrolling)
    - [Options Menu](#options-menu)
    - [Copy / Paste](#copy--paste)
    - [Selection Mode](#selection-mode)
    - [Detailed View](#detailed-view)
    - [Search](#search)
    - [File explorer](#file-explorer)
    - [Running commands](#running-commands)
  - [Key Management](#key-management-1)
    - [List](#list)
    - [Export](#export)
    - [Sign](#sign)
    - [Edit](#edit)
    - [Import/Receive](#importreceive)
    - [Send](#send)
    - [Generate](#generate)
    - [Delete](#delete)
    - [Refresh](#refresh)
  - [Styling](#styling)
    - [Colors](#colors)
    - [Splash screen](#splash-screen)
- [Roadmap](#roadmap)
  - [Platforms](#platforms)
  - [Packaging](#packaging)
  - [Command-Line Fallback](#command-line-fallback)
  - [Key Management Only](#key-management-only)
- [Resources](#resources)
  - [About the project](#about-the-project)
  - [External links](#external-links)
  - [In the media](#in-the-media)
- [Contact](#contact)
- [Funding](#funding)
- [License](#license)
- [Copyright](#copyright)

</details>

## Requirements

- [Rust](https://www.rust-lang.org/): `>=1.47.0`
- Core dependencies: `gnupg`, `gpgme>=1.7.0`, `libgpg-error`
- Other dependencies: `libxcb`

Pleases note that the name of these dependencies (packages) might change depending on the distribution/platform.([*](https://github.com/orhun/gpg-tui/issues/10))

For installing these dependencies:

- on Arch Linux, run `pacman -S gpgme libx11`
- on Debian/Ubuntu, run `apt-get install libgpgme-dev libx11-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev`
- on Fedora, run `dnf install gpgme-devel libX11-devel`
- on Void Linux, run `xbps-install -S gpgme-devel libxcb-devel libgpg-error-devel gnupg`

## Installation

<details>
  <summary>Packaging status</summary>

[![Packaging status](https://repology.org/badge/vertical-allrepos/gpg-tui.svg)](https://repology.org/project/gpg-tui/versions)

</details>

### Cargo

**gpg-tui** is available on [crates.io](https://crates.io/crates/gpg-tui/):

```sh
cargo install gpg-tui
```

### Arch Linux

#### Community

**gpg-tui** can be installed from the [community repository](https://archlinux.org/packages/community/x86_64/gpg-tui/) using [Pacman](https://wiki.archlinux.org/title/Pacman):

```sh
pacman -S gpg-tui
```

#### AUR

**gpg-tui** is also available on [AUR](https://aur.archlinux.org/packages/?O=0&SeB=b&K=gpg-tui&outdated=&SB=n&SO=a&PP=50&do_Search=Go) and it can be installed with an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers):

```sh
paru -S gpg-tui-git
```

Or if you prefer, you can clone the [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=gpg-tui&outdated=&SB=n&SO=a&PP=50&do_Search=Go) and compile them with [makepkg](https://wiki.archlinux.org/index.php/Makepkg):

```sh
# clone the AUR repository
git clone https://aur.archlinux.org/gpg-tui-git.git && cd gpg-tui-git/

# build the package
makepkg -si
```

### Gentoo

Available in [dm9pZCAq overlay](https://github.com/gentoo-mirror/dm9pZCAq)

```sh
eselect repository enable dm9pZCAq
emerge --sync dm9pZCAq
emerge app-crypt/gpg-tui::dm9pZCAq
```

### Homebrew

To install the [Homebrew package](https://formulae.brew.sh/formula/gpg-tui), run:

```sh
brew install gpg-tui
```

To update, run:

```sh
brew upgrade gpg-tui
```

### Docker

#### Docker Hub

See available [tags](https://hub.docker.com/r/orhunp/gpg-tui/tags).

```sh
docker pull orhunp/gpg-tui:[tag]
```

You can also use the following command for a quick launch:

```sh
docker run --rm -it -v "$HOME/.gnupg":/app/.gnupg --user 1000:1000 orhunp/gpg-tui --homedir /app/.gnupg
```

#### Using the Dockerfile

```sh
# clone the repository
git clone https://github.com/orhun/gpg-tui.git && cd gpg-tui/

# build the image
docker build -t gpg-tui .

# run the container
docker run -it gpg-tui
```

### FreeBSD

All required dependencies are automatically fetched and installed independently of the installation method chosen.

#### Building from source

```sh
# using a port
cd /usr/ports/security/gpg-tui
make install
```

```sh
# alternative method using portmaster
portmaster security/gpg-tui
```

#### Binary releases

```sh
# update repository catalogue (if outdated)
pkg update

# fetch and install the package
pkg install gpg-tui
```

### NetBSD

**gpg-tui** is available from the main [pkgsrc branch](https://pkgsrc.se/security/gpg-tui).

#### Install using the package manager

```sh
pkgin install gpg-tui
```

#### Building from source

```sh
cd /usr/pkgsrc/security/gpg-tui
make install
```

### Manually

#### Building from source

```sh
# clone the repository
git clone https://github.com/orhun/gpg-tui.git && cd gpg-tui/

# build and install
cargo install --root "$HOME/.cargo" --path .
```

#### Binary releases

See available [releases](https://github.com/orhun/gpg-tui/releases) that are automated by [Continuous Deployment](.github/workflows/cd.yml) workflow.

## Usage

```
gpg-tui [FLAGS] [OPTIONS]
```

```
FLAGS:
    -a, --armor      Enables ASCII armored output
        --splash     Shows the splash screen on startup
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```
OPTIONS:
        --config <path>        Sets the configuration file [env: GPG_TUI_CONFIG=]
        --homedir <dir>        Sets the GnuPG home directory [env: HOMEDIR=]
    -o, --outdir <dir>         Sets the output directory [env: OUTDIR=]
    -d, --default-key <key>    Sets the default key to sign with [env: DEFAULT_KEY=]
    -t, --tick-rate <ms>       Sets the tick rate of the terminal [env: TICK_RATE=]  [default: 250]
    -c, --color <color>        Sets the accent color of the terminal [env: COLOR=]  [default: gray]
    -s, --style <style>        Sets the style of the terminal [env: STYLE=]  [default: plain]
                               [possible values: plain, colored]
    -f, --file-explorer <app>  Sets the utility for file selection [env: FILE_EXPLORER=] [default: xplr]
        --select <option>      Enables the selection mode [env: SELECT=]
                               [possible values: key_id, key_fpr, user_id, row1, row2]
```

## Configuration

It is possible to override the command line arguments with a configuration file.

See [gpg-tui.toml](config/gpg-tui.toml) for the default configuration values.

The configuration file can be specified via `--config` argument or `GPG_TUI_CONFIG` environment variable. Also, it can be placed to a location where **gpg-tui** looks for:

* `<config_dir>` `/` `gpg-tui.toml`
* `<config_dir>` `/` `gpg-tui/gpg-tui.toml`
* `<config_dir>` `/` `gpg-tui/config`

`<config_dir>` depends on the platform as shown in the following table:

|Platform | Value                                 | Example                          |
| ------- | ------------------------------------- | -------------------------------- |
| Linux   | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config              |
| macOS   | `$HOME`/Library/Application Support   | /Users/Alice/Library/Application Support |
| Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming   |

## Key Bindings

### User Interface

| Key Binding          | Action                       |
| -------------------- | ---------------------------- |
| `?`                  | show help                    |
| `o,space,enter`      | show options                 |
| `hjkl,arrows,pgkeys` | navigate                     |
| `n`                  | switch to normal mode        |
| `v`                  | switch to visual mode        |
| `c`                  | switch to copy mode          |
| `p,C-v`              | paste from clipboard         |
| `a`                  | toggle armored output        |
| `1,2,3`              | set detail level             |
| `t,tab`              | toggle detail (all/selected) |
| `` ` ``              | toggle table margin          |
| `m`                  | toggle table size            |
| `C-s`                | toggle style                 |
| `/`                  | search                       |
| `:`                  | run command                  |
| `r,f5`               | refresh application          |
| `q,C-c/d,escape`     | quit application             |

### Key Management

| Key Binding   | Action        |
| ------------- | ------------- |
| `x`           | export key    |
| `s`           | sign key      |
| `e`           | edit key      |
| `i`           | import key(s) |
| `f`           | receive key   |
| `u`           | send key      |
| `g`           | generate key  |
| `d,backspace` | delete key    |
| `C-r`         | refresh keys  |

## Approach

Available keys in the keyring (which can be specified via `--homedir` argument) are showed on a table. This table consists of 2 columns which are [key information](#key-information) and [user information](#user-information).

The level of detail that an individual table row shows is determined by [detail levels](#detail-levels).

### Detail Levels

1. **Minimum**: shows only the primary key and user ID.

```
[sc--] rsa3072/B14085A20355B74DE0CE0FA1E19F76D037BD65B6  │  [u] Example Key <example@key>
```

2. **Standard**: shows all the subkeys and user IDs.

```
[sc--] rsa3072/B14085A20355B74DE0CE0FA1E19F76D037BD65B6  │  [u] Example Key <example@key>
|      └─(2021-05-14)                                    │   └─[u] Other User ID <example@key>
[--e-] rsa3072/E56CAC142AE5A979BEECB00FB4F68595CAD4E7E5  │
       └─(2021-05-14)
```

3. **Full**: shows signatures and notations.

```
[sc--] rsa3072/B14085A20355B74DE0CE0FA1E19F76D037BD65B6  │  [u] Example Key <example@key>
|      └─(2021-05-14)                                    │   │  └─[13] selfsig (2021-05-16)
[--e-] rsa3072/E56CAC142AE5A979BEECB00FB4F68595CAD4E7E5  │   │     └─[h] test@notation=xyz
       └─(2021-05-14)                                    |   └─[u] Other User ID <example@key>
                                                                   ├─[13] selfsig (2021-05-16)
                                                                   └─[10] 84C39331F6F85326 Other Signer Key <example@signer> (2021-05-16)
```

### Key Information

An example table entry for the detail level `full` (which includes subkeys) is explained via reference numbers below.

<pre>
[sc--]<b>⁰</b>  rsa3072<b>¹</b>/B14085A20355B74DE0CE0FA1E19F76D037BD65B6<b>²</b>
|<b>³</b>       └─(2021-05-14)<b>⁴</b>
[--e-]<b>⁰</b>*<b>⁶</b>rsa3072<b>¹</b>/E56CAC142AE5A979BEECB00FB4F68595CAD4E7E5<b>²</b>
         └─(2021-05-14) -> (2021-05-16)<b>⁴</b> [exp]<b>⁵</b>
</pre>

**0**: Key flags. Determines what the key can do.

* `s`: sign
* `c`: certify
* `e`: encrypt
* `a`: authenticate

**1**: Algorithm of the key.

**2**: Fingerprint of the key.

**3**: Indicates that the next key is a subkey.

**4**: Time information of the key:

* creation time (`Y-m-d`)
* expiration time (`Y-m-d`)

**5**: Is the key one of the following?

* `[exp]`: expired
* `[rev]`: revoked
* `[d]`: disabled
* `[i]`: invalid
* `[q]`: qualified

**6**: Star symbol (`*`) is shown after key flags if the key is selected as the default signing key.

### User Information

An example table entry for the detail level `full` (which includes other user IDs, signatures and notations) is explained via reference numbers below.

<pre>
[u]<b>⁰</b> Test Key &lt;test@test&gt;<b>¹</b>
 │<b>²</b>  └─[13]<b>³</b> selfsig<b>⁴</b> (2021-05-16)<b>⁶</b>
 │             └─[h]<b>⁹</b> test@notation=xyz<b>⁸</b>
 └─[u]<b>⁰</b> Test Key2 &lt;test2@test2&gt;<b>¹</b>
        ├─[13]<b>³</b> selfsig<b>⁴</b> (2021-05-16)<b>⁶</b>
        └─[10]<b>³</b> 84C39331F6F85326 Test Key 2 &lt;test2@example.com&gt;<b>⁵</b> (2021-05-16)<b>⁶</b> [!x]<b>⁷</b>
</pre>

**0**: Validity of the user.

* `[q]`: undefined
* `[n]`: never
* `[m]`: marginal
* `[f]`: full
* `[u]`: ultimate
* `[?]`: unknown

**1**: User ID. (`name` + `email`)

**2**: Indicates the next user ID.

**3**: Certification level of the signature.

* `[10]`: no indication
* `[11]`: personal belief but no verification
* `[12]`: casual verification
* `[13]`: extensive verification

**4**: Indicates that this is a self signature, whereby the users' own private key was used to sign their public key.

**5**: Key and user ID of the signer. (`key` + `name` + `email`)

**6**: Time information of the signature.

* creation time (`Y-m-d`)
* expiration time (`Y-m-d`)

**7**: Is the signature one of the following?

* `[exp]`: expired
* `[rev]`: revoked
* `[i]`: invalid
* `[!x]`: non-exportable

**8**: Notation data.

**9**: Flags associated with the notation data.

* `[h]`: the notation data is in human readable form
* `[!]`: the notation data is critical

## Features

Press `?` while running the terminal interface to see information about key bindings and GnuPG configuration.

![](demo/gpg-tui-help_menu.gif)

### User Interface

#### Scrolling

Use arrow or `hjkl` keys to scroll and navigate through lists/menus/tabs. Additionally, you can use `Ctrl-Up/Down` combinations or `PageUp`/`PageDown` keys to scroll to the top/bottom.

![](demo/gpg-tui-scrolling.gif)

If rows are not fitting on the terminal interface, use `Alt` + arrow/`hjkl` keys to individually scroll them.

![](demo/gpg-tui-scrolling_rows.gif)

![](demo/gpg-tui-scrolling_rows2.gif)

#### Options Menu

Most of the actions can be performed using the options menu. Simply press `Enter` and select what you want to do.

![](demo/gpg-tui-options_menu.gif)

#### Copy / Paste

There's a `copy` mode for making it easier to copy particular values to the clipboard. To use this mode, press `c` followed by one of the key bindings:

* `x`: Copy the exported key
* `i`: Copy the key id
* `f`: Copy the key fingerprint
* `u`: Copy the user id
* `1,2`: Copy the content of the row

Then the value will be copied to the clipboard and the application mode will be reverted to `normal`.

Press `ESC` or `n` to cancel and switch to `normal` mode during this operation.

![](demo/gpg-tui-copy_mode.gif)

Instead of copying values with `copy` mode, you can use the `visual` mode which disables the mouse capture. It means that you can select/highlight the text on the interface and copy as you do normally.

`visual` mode can be used for other purposes such as scrolling via mouse.

![](demo/gpg-tui-visual_mode.gif)

#### Selection Mode

In the selection mode, key bindings that are responsible for showing the options menu (e.g. `enter`) are used for exiting the user interface and printing out the selection to the standard output. This is useful when you want to use **gpg-tui** in conjunction with shell commands/other tools.

For switching to the selection mode, use the `--select` argument as follows:

```sh
gpg-tui --select <option>
```

`<option>` might be one of the following:

- `key_id`: Key ID
- `key_fpr`: Key fingerprint
- `user_id`: User ID
- `row<n>`: Contents of the nth row

![](demo/gpg-tui-selection_mode.gif)

For example, you can use the following shell function to encrypt a file for the selected recipient (key ID):

```sh
function encrypt() { gpg -e -r $(gpg-tui --select key_id) "$@"; }
```

#### Detailed View

Press `Tab` to toggle the [detail level](#detail-levels) for the selected entry in the list. Number keys (e.g. `1`, `2`, `3`) can be also used to set a specific level.

![](demo/gpg-tui-detailed_view.gif)

Press `t` to toggle the detail level for all entries in the list.

![](demo/gpg-tui-detailed_view2.gif)

There are couple a of different modes for the size of the tables which changes the details that each entry shows. You can use the `m` key for switching to different modes.

![](demo/gpg-tui-detailed_view3.gif)

#### Search

Press `/` to search for a value from the currently shown table.

![](demo/gpg-tui-search.gif)

#### File explorer

Some of the key management operations such as [importing keys](#importreceive) _optionally_ use a file explorer utility. As default, **gpg-tui** uses [xplr](https://github.com/sayanarijit/xplr) if the `xplr` binary is [installed](https://github.com/sayanarijit/xplr/wiki/Quickstart) on the system. To change which utility is going to be used, `--file-explorer` argument can be used or it can be specified in the [configuration file](#configuration).

```sh
gpg-tui --file-explorer "fzf --color dark"
```

#### Running commands

Every operation on the terminal interface is handled implicitly by the application-specific commands. So it's possible to do certain things by switching to command mode with pressing `:` and running commands. (similar to Vim)

For example,

* `:list pub` -> list public keys
* `:set armor true` -> enable armored output

A full list of commands can be found [here.](COMMANDS.md)

![](demo/gpg-tui-running_commands.gif)

Also you can switch between command mode and search by pressing `Tab`.

![](demo/gpg-tui-switch_search.gif)

### Key Management

#### List

Available keys in the keyring are listed on a table as default. They can be [scrolled](#scrolling) or the listing type (public/secret keys) can be changed by changing the tab via arrow keys.

See the [approach](#approach) section for more information about the meaning of the table rows.

![](demo/gpg-tui-list_keys.gif)

#### Export

Press `x` to export the selected key to a file. The default output directory is `$GNUPGHOME/out` and can be changed by either using `--homedir` or `--outdir` argument.

Additionally, you can enable/disable armored output by pressing `a`.

![](demo/gpg-tui-export_key.gif)

Also, you can export the secret subkeys by using the options menu:

![](demo/gpg-tui-export_subkeys.gif)

#### Sign

Press `s` to sign the selected key with the default secret key. This key can be specified with `--default-key` argument or using the options menu.

This feature uses `gpg` fallback and runs `gpg --sign-key` command.

![](demo/gpg-tui-sign_key.gif)

#### Edit

Press `e` to edit the selected key.

This feature uses `gpg` fallback and runs `gpg --edit-key` command. It presents a menu that provides a list of options to change the key details. See the [edit-key](https://www.gnupg.org/gph/en/manual/r899.html) documentation for more information.

![](demo/gpg-tui-edit_key.gif)

#### Import/Receive

Import operation uses a [file explorer](#file-explorer) for selecting the key(s) to import. Press `i` to launch the file explorer (defaults to `xplr`), and select the key file(s) to import:

![](demo/gpg-tui-import_key_xplr.gif)

If a file explorer is not specified or installed, import operation is done by using the `:import` command. So press `i` to switch to command mode (which will automatically add the import command) and then give it your file(s) to import.

![](demo/gpg-tui-import_key.gif)

You can also import keys from clipboard using `:import-clipboard` command or the options menu.

![](demo/gpg-tui-import_key_clipboard.gif)

Similar to import, receive operation is also done by using a command which is `:receive`. So press `f` (for **f**etching keys from a keyserver) and give it your key ID(s).

This feature uses `gpg` fallback and runs `gpg --receive-keys` command.

![](demo/gpg-tui-receive_key.gif)

#### Send

Press `u` (for **u**ploading to the keyserver) followed by `y` (for confirmation) to send the selected key to the default keyserver.

![](demo/gpg-tui-send_key.gif)

#### Generate

Press `g` to generate a new key pair.

This feature uses `gpg` fallback and runs `gpg --full-generate-key` command. It presents dialogs for all of the generation options.

![](demo/gpg-tui-generate_key.gif)

#### Delete

Press `Backspace` followed by `y` (for confirmation) to delete the selected key from the keyring.

![](demo/gpg-tui-delete_key.gif)

#### Refresh

Press `Ctrl-y` for refreshing the keyring.

This feature uses `gpg` fallback and runs `gpg --refresh-keys` command.

![](demo/gpg-tui-refresh_keys.gif)

### Styling

You can customize the look of **gpg-tui** to get rid of its _boring_ and _minimalistic_ vibe. (!)

#### Colors

To enable colors, you can specify a style with `--style` argument or press `Ctrl-S` while running for toggling the style. Currently, only one style is supported which is `colored`.

```sh
gpg-tui --style colored
```

![](demo/gpg-tui-colored_style.gif)

If the default _accent_ color of the interface causes problems with your theme or if you just want to change it to something more vivid, you can use `--color` argument to specify another color in HEX format.

```sh
gpg-tui --style colored --color 507030
```

![](demo/gpg-tui-custom_colors.gif)

#### Splash screen

There is a splash screen that shows the project's logo for a couple of seconds if `--splash` flag is present. It's purely cosmetical.

```sh
gpg-tui --splash
```

![](demo/gpg-tui-splash_screen.gif)

To enable colors for the splash screen, use the `colored` style.

```sh
gpg-tui --splash --style colored
```

![](demo/gpg-tui-splash_screen_colored.gif)

## Roadmap

### Platforms

**gpg-tui** is tested on Linux systems during the development phase. It should be tested on other platforms such as Microsoft Windows and macOS and found issues should be reported for future compatibility with these platforms.

### Packaging

[![Packaging status](https://img.shields.io/repology/repositories/gpg-tui?label=in%20repositories&style=flat&labelColor=424242&color=1d1d1d&logoColor=white)](https://repology.org/project/gpg-tui/versions)

**gpg-tui** should be more and easily accessible for other platforms/distributions. Thus, it should be packaged for package managers such as [Homebrew](https://brew.sh/) and [APT](https://en.wikipedia.org/wiki/APT_(software)). If you're a packager and want to contribute, feel free to [submit an issue](https://github.com/orhun/gpg-tui/issues/new/choose) or [start a discussion](https://github.com/orhun/gpg-tui/discussions/new)!

### Command-Line Fallback

Some of the features of **gpg-tui** require the execution of the CLI program `gpg` in order to operate. This is due to the fact that designing a TUI for the menus that gpg already provides is redundant and time-consuming. Also, in case these menus will change in the future, it is better to rely on the gpg rather than implementing these options using GPGME. On the other hand, gpg has some commands that GPGME doesn't directly provide. (e.g `--refresh-keys`) So it is more convenient to utilize gpg for these cases.

The plan for the future of **gpg-tui** is utilizing `gpg` _when it is necessary_, depending on whether if it is more convenient for the user.

### Key Management Only

**gpg-tui** only aims to do **key management** for now, although it can do much more utilizing GPGME and/or `gpg`. It's due to the design choice and also for setting the boundaries of the project.

## Resources

### About the project

* [Code of Conduct](./CODE_OF_CONDUCT.md)
* [Contribution Guidelines](./CONTRIBUTING.md)
* [Changelog](./CHANGELOG.md)
* [List of Commands](./COMMANDS.md)

### External links

* [GnuPG Documentation](https://gnupg.org/documentation/)
* [Understanding PGP/GPG Key Server Output](https://www.jamieweb.net/blog/understanding-pgp-gpg-key-server-output/)
* [What is the meaning of GnuPG's --list-sigs output?](https://security.stackexchange.com/questions/141501/what-is-the-meaning-of-gnupgs-list-sigs-output)

### In the media

- [Hacker News](https://news.ycombinator.com/item?id=27329598)
- [heise online - IT-News](https://www.heise.de/news/gpg-tui-GnuPG-Schluessel-bequem-im-Terminal-verwalten-6057698.html)
- [News Break](https://www.newsbreak.com/news/2266206071969/introducing-gpg-tui-a-terminal-user-interface-for-gnupg)

## Contact

* [![Join Matrix Room](https://img.shields.io/matrix/gpg-tui:matrix.org?style=flat&labelColor=1d1d1d&logo=matrix&logoColor=white&color=424242&label=join%20matrix)](https://matrix.to/#/!WRksRGtlvyEvEDByqt:matrix.org?via=matrix.org)
* [![Follow @gpg_tui](https://img.shields.io/twitter/follow/gpg_tui?style=flat&labelColor=1d1d1d&logo=twitter&logoColor=white&color=424242)](https://twitter.com/gpg_tui)
* [![https://orhun.dev](https://img.shields.io/badge/author-orhun-000000?style=flat&labelColor=1d1d1d&color=424242&logo=Rust&logoColor=white)](https://orhun.dev)
  * [![Follow @orhun](https://img.shields.io/github/followers/orhun?label=follow%20%40orhun&style=flat&labelColor=1d1d1d&logo=GitHub&logoColor=white&color=424242)](https://github.com/orhun)
  * [![Follow @orhunp_](https://img.shields.io/twitter/follow/orhunp_?style=flat&labelColor=1d1d1d&logo=twitter&logoColor=white&color=424242)](https://twitter.com/orhunp_)

## Funding

If you find **gpg-tui** and/or other projects on my [GitHub profile](https://github.com/orhun/) useful, consider [becoming a patron](https://www.patreon.com/join/orhunp)!

[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpatrons&style=flat&labelColor=424242&color=1d1d1d&logo=Patreon&logoColor=white)](https://patreon.com/join/orhunp)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpledges&style=flat&labelColor=424242&color=1d1d1d&logo=Patreon&logoColor=white&label=)](https://patreon.com/join/orhunp)

## License

[The MIT License](https://opensource.org/licenses/MIT)

## Copyright

Copyright © 2021, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
