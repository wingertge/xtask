<div align="center">
<img src="https://raw.githubusercontent.com/tracel-ai/xtask/main/assets/xtask.png" width="256px"/>

<h1>Tracel Xtask</h1>

[![Discord](https://img.shields.io/discord/1038839012602941528.svg?color=7289da&&logo=discord)](https://discord.gg/uPEBbYYDB6)
[![Current Crates.io Version](https://img.shields.io/crates/v/tracel-xtask)](https://crates.io/crates/tracel-xtask)
[![Minimum Supported Rust Version](https://img.shields.io/crates/msrv/tracel-xtask)](https://crates.io/crates/tracel-xtask)
[![Test Status](https://github.com/tracel-ai/xtask/actions/workflows/ci.yml/badge.svg)](https://github.com/tracel-ai/xtask/actions/workflows/ci.yml)
![license](https://shields.io/badge/license-MIT%2FApache--2.0-blue)

---

<br/>
</div>

A collection of easy-to-use and extensible commands to be used in your [xtask CLI][1] based on [clap][2].

We rely on these commands in each of our Tracel repositories. By centralizing our redundant commands we save a big amount
of code duplication, boilerplate and considerably lower their maintenance cost. This also provides a unified interface across
all of our repositories.

These commands are not specific to Tracel repositories and they should be pretty much usable in any Rust repositories with
a cargo workspace as well as other repositories where Rust is not necessarily the only language. The commands can be easily
extended using handy proc macros and by following some patterns described in this README.

## Getting Started

### Setting Up a Cargo Workspace with an xtask binary crate

1. **Create a new Cargo workspace:**

```bash
cargo new my_workspace --vcs none
cd my_workspace
```

2. **Create the `xtask` binary crate:**

```bash
cargo new xtask --bin --vcs none
```

3. **Configure the workspace:**

Edit the `Cargo.toml` file in the root of the workspace to include the following:

```toml
[workspace]
members = ["xtask"]
```

4. **Add the `tracel-xtask` dependency:**

In the `xtask/Cargo.toml` file, add the following under `[dependencies]`:

```toml
[dependencies]
tracel-xtask = "~1.0"
```

5. **Build the workspace:**

 ```bash
 cargo build
 ```

Your workspace is now set up with a `xtask` binary crate that depends on `tracel-xtask` version 1.0.x.

### Bootstrap main.rs

1. In the `main.rs` file of the newly created `xtask` crate, import the `tracel_xtask` prelude moduel and then declare
   a `Command` enum. Select the base commands you want to use by adding the `macros::base_commands` attribute:

```rust
use tracel_xtask::prelude::*;

#[macros::base_commands(
    Bump,
    Check,
    Fix
    Test,
)]
pub enum Command {}
```

2. Update the `main` function to initialize the xtask parser and dispatch the base commands:

```rust
fn main() -> anyhow::Result<()> {
    let args = init_xtask::<Command>()?;
    match args.command {
        // dispatch_base_commands function is generated by the commands macro
        _ => dispatch_base_commands(args),
    }
}
```

3. Build the workspace with `cargo build` at the root of the repository to verify that everything is.


4. You should now be able to display the main help screen which lists the commands you selected previously:

```sh
cargo xtask --help
```

### Setup aliases for easy invocation

#### Cargo Alias

Invoking the xtask binary with cargo is very verbose and not really usable as is. Happily we can create a
cargo alias to make it really effortless to invoke it.

Create a new file `.cargo/config.toml` in your repository with the following contents:

```toml
[alias]
xtask = "run --target-dir target/xtask --package xtask --bin xtask --"
```

This saves quite a few characters to type as you can now invoke `xtask` direclty like this:

```sh
cargo xtask
```

Try it with `cargo xtask --help`.

#### Shell Alias

We can save even more typing by creating a shell alias for `cargo xtask`.

For instance we can set the alias to `cx`. Here is how to do it in various shells.

- For bash:

```bash
nano ~/.bashrc

# add this to the file
alias cx='cargo xtask'

# save and source the file or restart the shell session
source ~/.bashrc
```

- For zsh:

```sh
nano ~/.zshrc

# add this to the file
alias cx='cargo xtask'

# save and source the file or restart the shell session
source ~/.zshrc
```

- For fish:

```fish
nano ~/.config/fish/config.fish

# add this to the file
alias cx='cargo xtask'

# save and source the file or restart the shell session
source ~/.config/fish/config.fish
```

- For powershell:

```powershell
notepad $PROFILE

# add this at the end of file
function cx {
    cargo xtask $args
}

# save and quit then open a new powershell terminal
```

Try it with `cx --help` at the root of the repository.

## Conventions

### Repository structure

All our repositories follow the same directory hierarchy:
- a `crates` directory which contains all the crates of the workspace
- an `examples` directory which holds all the examples crates
- a `xtask` directory which is the binary crate for our xtask CLI using `tracel-xtask`

### About tests

As per Cargo convention, [Integration tests][3] are tests contained in a `tests` directory of a crate besides its `src` directory.

Inline tests in `src` directory are called [Unit tests][4].

`tracel-xtask` allow to easily execute them separately using the `test` command.

## Interface generalities

### Target

There are 4 default targets provided by `tracel-xtask`:
- `workspace` which targets the cargo workspace, this is the default target
- `crates` are all the binary crates and library crates
- `examples` are all the example crates
- `all-packages` are both `crates` and `examples` targets

`workspace` and `all-packages` are different because `workspace` uses the `--workspace` flag of cargo whereas `all-packages`
relies on `crates` and `examples` targets which use the `--package` flag. So `all-packages` executes a command for each crate
or example individually.

Here are some examples:

```sh
# run all the crates tests
cargo xtask test --target crates all
# check format for examples, binaries and libs
cargo xtask check --target all-packages unit
# build the workspace
cargo xtask build --target workspace
# workspace is the default target so this has the same effect
cargo xtask build
```

### Global options

The following options are global and precede the actual command on the command line:

- Environment (`-e`, `--environment`):

```sh
cargo xtask -e production build
```

`-e` or `--environment` does not do anything per se in the base commands, it is a flag whose only goal is
to inform your custom commands or dispatch functions about the targeted environment which can be `development` (default),
`staging` or `production`.

- Execution environment (`-E`, `--execution-environment`):

```sh
cargo xtask -E no-std build
```

`-E` or `--execution-environment` does not do anything per se in the base commands, it is a flag whose only goal is
to inform your custom commands or dispatch functions about the targeted execution environment which can be `std` or
`no-std`.

- Coverage (`-c`, `--enable-coverage`):

`-c` or `--enabled-coverage` setups the Rust toolchain to generate coverage information.

## Anatomy of a base command

We use the derive API of clap which is based on structs, enums and attribute proc macros. Each base command is a
submodule of the `base_commands` module. If the command accepts arguments there is a corresponding struct named `<command>CmdArgs`
which declare the options, arguments and subcommands. In the case of subcommands a corresponding enum named `<command>SubCommand`
is defined.

Here is an example with a `foo` command:

```rust
#[macros::declare_command_args(Target, FooSubCommand)]
struct FooCmdArgs {}

pub enum FooSubCommand {
    /// A sub command for foo (usage on the command line: cargo xtask foo print-something)
    PrintSomething,
}
```

Note that it is possible to have an arbitrary level of nested subcommands but deeper nested subcommands cannot be extended,
in other words, only the first level of subcommands can be extended. If possible, try to design commands with only one
level of subcommands to keep the interface simple.

In the following sections we will see how to create completely new commands as well how to extend existing base commands.

## Customization

### Create a new command

1. First, we organize commands by creating a `commands` module. Create a file `xtask/src/commands/mycommand.rs` as well
   as the corresponding `mod.rs` file to declare the module contents.

2. Then, in `mycommand.rs` define the arguments struct with the `declare_command_args` macro and define the `handle_command`
   function. The `declare_command_args` macro takes two parameters, the first is the type of the target enum and the second
   is the type of the subcommand enum if any. If the command has no target or no subcommand then put `None` for each argument
   respectively. `Target` is the default target type provided by `tracel-xtask`. This type can be extended to support more
   targets as we will see in a later section.

```rust
use tracel_xtask::prelude::*;

#[macros::declare_command_args(Target, None)]
struct MyCommandCmdArgs {}

pub fn handle_command(_args: MyCommandCmdArgs) -> anyhow::Result<()> {
    println!("Hello from my-command");
    Ok(())
}
```

3. Make sure to update the `mod.rs` file to declare the command module:

```
pub(crate) mod my_command;
```

4. We can now add a new variant to the `Command` enum in `main.rs`:

```rust
mod commands;

use tracel_xtask::prelude::*;

#[macros::base_commands(
    Bump,
    Check,
    Fix,
    Test,
)]
pub enum Command {
    MyCommand(commands::mycommand::MyCommandCmdArgs),
}
```

5. And dispatch its handling to our new command module:

```rust
fn main() -> anyhow::Result<()> {
    let args = init_xtask::<Command>()?;
    match args.command {
        Command::NewCommand(args) => commands::new_command::handle_command(args),
        _ => dispatch_base_commands(args),
    }
}
```

6. You can now test your new command with:

```sh
cargo xtask my-command --help

cargo xtask my-command
```

### Extend the default Target enum

Let's implement a new command called `extended-target` to illustrate how to extend the default `Target` enum.

1. Create a `commands/extended_target.rs` file and update the `mod.rs` file as we saw in the previous section.

2. We also need to add a new `strum` dependency to our `Cargo.toml` file:

```toml
[dependencies]
strum = {version = "0.26.3", features = ["derive"]}
```

3. Then we can extend the `Target` enum with the `macros::extend_targets` attribute in our `extended_target.rs` file.
   Here we choose to add a new target called `frontend` which targets the frontend component we could find for instance
   in a monorepo:

```rust
use tracel_xtask::prelude::*;

#[macros::extend_targets]
pub enum MyTarget {
    /// Target the frontend component of the monorepo.
    Frontend,
}
```

4. Then we define our command arguments by referencing our newly created `MyTarget` enum in the `declare_command_args` attribute:

```rust
#[macros::declare_command_args(MyTarget, None)]
struct ExtendedTargetCmdArgs {}
```

5. Our new target is then available for use in the `handle_command` function:

```rust
pub fn handle_command(args: ExtendedTargetCmdArgs) -> anyhow::Result<()> {
    match args.target {
        // Default targets
        MyTarget::AllPackages => println!("You chose the target: all-packages"),
        MyTarget::Crates => println!("You chose the target: crates"),
        MyTarget::Examples => println!("You chose the target: examples"),
        MyTarget::Workspace => println!("You chose the target: workspace"),

        // Additional target
        MyTarget::Frontend => println!("You chose the target: frontend"),
    };
    Ok(())
}
```

6. Register our new command the usual way by adding it to our `Command` enum and dispatch it
   in the `main` function:

```rust
mod commands;

use tracel_xtask::prelude::*;

#[macros::base_commands(
    Bump,
    Check,
    Fix,
    Test,
)]
pub enum Command {
    ExtendedTarget(commands::extended_target::ExtendedTargetCmdArgs),
}

fn main() -> anyhow::Result<()> {
    let args = init_xtask::<Command>()?;
    match args.command {
        Command::ExtendedTarget(args) => commands::extended_target::handle_command(args),
        _ => dispatch_base_commands(args),
    }
}
```

7. Test the command with:

```rust
cargo xtask extended-target --help

cargo xtask extended-target --target frontend
```

### Extend a base command

To extend an existing command we use the `macros::extend_command_args` attribute which takes three parameters:
- first argument is the type of the base command arguments struct to extend,
- second argument is the target type (or `None` if there is no target),
- third argument is the subcommand type (or `None` if there is no subcommand).

Let's use two examples to illustrate this, the first is a command to extend the `build` base command with
a new `--debug` argument; and the second is a new command to extend the subcommands of the `check` base command
to add a new `my-check` subcommand.

Note that you can find more examples in the `xtask` crate of this repository.

#### Extend the arguments of a base command

We create a new command called `extended-build-args` which will have an additional argument called `--debug`.

1. Create the `commands/extended_build_args.rs` file and update the `mod.rs` file as we saw in the previous section.

2. Extend the `BuildCommandArgs` struct using the attribute `macros::extend_command_args` and define the `handle_command` function.
   Note that the macro automatically implements the `TryInto` trait which makes it easy to dispatch back to the base command
   own `handle_command` function. Also note that if the base command requires a target then you need to provide a target as well
   in your extension, i.e. the target parameter of the macro cannot be `None` if the base command has a `Target`.

```rust
use tracel_xtask::prelude::*;

#[macros::extend_command_args(BuildCmdArgs, Target, None)]
pub struct ExtendedBuildArgsCmdArgs {
    /// Print additional debug info when set
    #[arg(short, long)]
    pub debug: bool,
}

pub fn handle_command(args: ExtendedBuildArgsCmdArgs) -> anyhow::Result<()> {
    if args.debug {
        println!("Debug is enabled");
    }
    base_commands::build::handle_command(args.try_into().unwrap())
}
```

3. Register the new command the usual way by adding it to the `Command` enum and dispatch it
   in the `main` function:

```rust
mod commands;

use tracel_xtask::prelude::*;

#[macros::base_commands(
    Bump,
    Check,
    Fix,
    Test,
)]
pub enum Command {
    ExtendedBuildArgs(commands::extended_build_args::ExtendedBuildArgsCmdArgs),
}

fn main() -> anyhow::Result<()> {
    let args = init_xtask::<Command>()?;
    match args.command {
        Command::ExtendedBuildArgs(args) => commands::extended_build_args::handle_command(args),
        _ => dispatch_base_commands(args),
    }
}
```

4. Test the command with:

```rust
cargo xtask extended-build-args --help

cargo xtask extended-build-args --debug
```

#### Extend the subcommands of a base command

For this one we create a new command called `extended-check-subcommands` which will have an additional subcommand.

1. Create a `commands/extended_check_subcommands.rs` file and update the `mod.rs` file as we saw in the previous section.

2. Extend the `CheckCommandArgs` struct using the attribute `macros::extend_command_args`:

```rust
use tracel_xtask::prelude::*;

#[macros::extend_command_args(CheckCmdArgs, Target, ExtendedCheckSubcommand)]
pub struct ExtendedCheckedArgsCmdArgs {}
```

3. Implement the `ExtendedCheckSubcommand` enum by extending the `CheckSubcommand` base enum with the macro `extend_subcommands`.
   It takes the name of the type of the subcommand enum to extend:

```rust
#[macros::extend_subcommands(CheckSubCommand)]
pub enum ExtendedCheckSubcommand {
    /// An additional subcommand for our extended check command.
    MySubcommand,
}
```

4. Implement the `handle_command` function to handle the new subcommand. Note that we must handle the `All` subcommand as well:

```rust
use strum::IntoEnumIterator;

pub fn handle_command(args: ExtendedCheckedArgsCmdArgs) -> anyhow::Result<()> {
    match args.get_command() {
        ExtendedCheckSubcommand::MySubcommand => run_my_subcommand(args.clone()),
        ExtendedCheckSubcommand::All => {
            ExtendedCheckSubcommand::iter()
                .filter(|c| *c != ExtendedCheckSubcommand::All)
                .try_for_each(|c| {
                    handle_command(
                        ExtendedCheckedArgsCmdArgs {
                            command: Some(c),
                            target: args.target.clone(),
                            exclude: args.exclude.clone(),
                            only: args.only.clone(),
                        },
                    )
                })
        }
        _ => base_commands::check::handle_command(args.try_into().unwrap()),
    }
}

fn run_my_subcommand(_args: ExtendedCheckedArgsCmdArgs) -> Result<(), anyhow::Error> {
    println!("Executing new subcommand");
    Ok(())
}
```

5. Register the new command the usual way by adding it to the `Command` enum and dispatch it
   in the `main` function:

```rust
mod commands;

use tracel_xtask::prelude::*;

#[macros::base_commands(
    Bump,
    Check,
    Fix,
    Test,
)]
pub enum Command {
    ExtendedCheckSubcommand(commands::extended_check_subcommands::ExtendedCheckedArgsCmdArgs),
}

fn main() -> anyhow::Result<()> {
    let args = init_xtask::<Command>()?;
    match args.command {
        Command::ExtendedCheckSubcommand(args) => commands::extended_check_subcommands::handle_command(args),
        _ => dispatch_base_commands(args),
    }
}
```

6. Test the command with:

```rust
cargo xtask extended-check-subcommands --help

cargo xtask extended-check-subcommands my-check
```

## Custom builds and tests

`tracel-xtask` provides helper functions to easily execute custom builds or tests with specific features or build targets (do not confuse
Rust build targets which is an argument of the `cargo build` command with the xtask target we introduced previously).

For instance we can extend the `build` command to build additional crates with custom features or build targets using the helper function:

```rust
pub fn handle_command(mut args: tracel_xtask::commands::build::BuildCmdArgs)  -> anyhow::Result<()> {
    // regular execution of the build command
    tracel_xtask::commands::build::handle_command(args)?;

    // additional crate builds
    // build 'my-crate' with all the features
    tracel_xtask::utils::helpers::custom_crates_build(vec!["my-crate"], vec!["--all-features"], None, None, "all features")?;
    // build 'my-crate' with specific features
    tracel_xtask::utils::helpers::custom_crates_build(vec!["my-crate"], vec!["--features", "myfeature1,myfeature2"], None, None, "myfeature1,myfeature2")?;
    // build 'my-crate' with a different target than the default one
    tracel_xtask::utils::helpers::custom_crates_build(vec!["my-crate"], vec!["--target", "thumbv7m-none-eabi"], None, None, "thumbv7m-none-eabi target")?;
    Ok(())
}
```

## Enable and generate coverage information

Here is a example GitHub job which shows how to setup coverage, enable it and upload coverage information to codecov:

```yaml
env:
  GRCOV_LINK: "https://github.com/mozilla/grcov/releases/download"
  GRCOV_VERSION: "0.8.19"

jobs:
  my-job:
    runs-on: ubuntu-22.04
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: install rust
        uses: dtolnay/rust-toolchain@master
        with:
          components: rustfmt, clippy
          toolchain: stable
      - name: Install grcov
        shell: bash
        run: |
          curl -L "$GRCOV_LINK/v$GRCOV_VERSION/grcov-x86_64-unknown-linux-musl.tar.bz2" |
          tar xj -C $HOME/.cargo/bin
          cargo xtask coverage install
      - name: Build
        shell: bash
        run: cargo xtask build
      - name: Tests
        shell: bash
        run: cargo xtask --enable-coverage test all
      - name: Generate lcov.info
        shell: bash
        # /* is to exclude std library code coverage from analysis
        run: cargo xtask coverage generate --ignore "/*,xtask/*,examples/*"
      - name: Codecov upload lcov.info
        uses: codecov/codecov-action@v4
        with:
          files: lcov.info
          token: ${{ secrets.CODECOV_TOKEN }}
```

## Special command 'validate'

By convention this command is responsible to run all the checks, builds, and/or tests that validate the code
before opening a pull request or merge request.

The command `Validate` can been added via the macro `tracel_xtask_macros::commands` like the other commands.

By default all the checks from the `check` command are run as well as both unit and integration tests from
the `test` command.

You can make your own `handle_command` function if you need to perform more validations. Ideally this function
should only call the other commands `handle_command` functions.

For quick reference here is a simple example to perform all checks and tests against the workspace:

```rust
pub fn handle_command(args: ValidateCmdArgs) -> anyhow::Result<()> {
    let target = Target::Workspace;
    let exclude = vec![];
    let only = vec![];

    // checks
    [
        CheckSubCommand::Audit,
        CheckSubCommand::Format,
        CheckSubCommand::Lint,
        CheckSubCommand::Typos,
    ]
    .iter()
    .try_for_each(|c| {
        super::check::handle_command(CheckCmdArgs {
            target: target.clone(),
            exclude: exclude.clone(),
            only: only.clone(),
            command: Some(c.clone()),
            ignore_audit: args.ignore_audit,
        })
    })?;

    // tests
    super::test::handle_command(TestCmdArgs {
        target: target.clone(),
        exclude: exclude.clone(),
        only: only.clone(),
        threads: None,
        jobs: None,
        command: Some(TestSubCommand::All),
    })?;

    Ok(())
}
```

## Base commands list

### Check and Fix

The `check` and `fix` commands are designed to help you maintain code quality during development.
They run various checks and fix issues, ensuring that your code is clean and follows best practices.

`check` and `fix` contains the same subcommands to audit, format, lint or proofread a code base.

While the `check` command only reports issues, the `fix` command attempts to fix them as they are encountered.

Each check can be executed separately or all of them can be executed sequentially using `all`.

Usage to lint the code base:

```sh
cargo xtask check lint

cargo xtask fix lint

cargo xtask fix all
```

### Running Tests

Testing is a crucial part of development, and the `test` command is designed to make this process easy.

This command makes the distinction between unit tests and integrations tests. [Unit tests][4] are inline tests under the
`src` directory of a crate. [Integration tests][3] are tests defined in files under the `tests` directory of a crate besides
the `src` directory.

Usage:
```sh
# execute workspace unit tests
cargo xtask test unit
# execute workspace integration tests
cargo xtask test integration
# execute workspace both unit tests and integration tests
cargo xtask test all
```

Note that documentation tests are supported by the `doc` command.

### Documentation

Command to build and test the documentation in a workspace.

### Bumping Versions

This is a command reserved for repository maintainers.

The `bump` command is used to update the version numbers of all first-party crates in the repository.
This is particularly useful when you're preparing for a new release and need to ensure that all crates have the correct version.

You can bump the version by major, minor, or patch levels, depending on the changes made.
For example, if you’ve made breaking changes, you should bump the major version.
For new features that are backwards compatible, bump the minor version.
For bug fixes, bump the patch version.

Usage:
```sh
cargo xtask bump <SUBCOMMAND>
```

### Publishing Crates

This is a command reserved for repository maintainers and is tipically used in `publish` GitHub workflows.

This command automates the process of publishing crates to `crates.io`, the Rust package registry.
By specifying the name of the crate, `xtask` handles the publication process, ensuring that the crate is available for others to use.

Usage:
```sh
cargo xtask publish <NAME>
```

As mentioned, this command is often used in a GitHub workflow.
We provide a Tracel's reusable [publish-crate][8] workflow that makes use of this command.
Here is a simple example with a workflow that publishes two crates A and B with A depending on B.

```yaml
name: publish all crates

on:
  push:
    tags:
      - "v*"

jobs:
  publish-B:
    uses: tracel-ai/github-actions/.github/workflows/publish-crate.yml@v1
    with:
      crate: B
    secrets:
      CRATES_IO_API_TOKEN: ${{ secrets.CRATES_IO_API_TOKEN }}

  # --------------------------------------------------------------------------------
  publish-A:
    uses: tracel-ai/github-actions/.github/workflows/publish-crate.yml@v1
    with:
      crate: A
    needs:
      - publish-B
    secrets:
      CRATES_IO_API_TOKEN: ${{ secrets.CRATES_IO_API_TOKEN }}
```

### Coverage

This command provide a subcommand to install the necessary dependencies for performing code coverage and a subcommand to generate the
coverage info file that can then be uploaded to a service provider like codecov. See dedicated section `Enable and generate coverage information`.

### Dependencies

Various additional subcommands about dependencies.

`deny` make sure that all dependencies meet requirements using [cargo-deny][5].

`unused` detects dependencies in the workspace that are not in ussed.

### Vulnerabilities

This command makes it easier to execute sanitizers as described in [the Rust unstable book][6].

These sanitizers require a nightly toolchain.

```
Run the specified vulnerability check locally. These commands must be called with 'cargo +nightly'

Usage: xtask vulnerabilities <COMMAND>

Commands:
  all                            Run all most useful vulnerability checks
  address-sanitizer              Run Address sanitizer (memory error detector)
  control-flow-integrity         Run LLVM Control Flow Integrity (CFI) (provides forward-edge control flow protection)
  hw-address-sanitizer           Run newer variant of Address sanitizer (memory error detector similar to AddressSanitizer, but based on partial hardware assistance)
  kernel-control-flow-integrity  Run Kernel LLVM Control Flow Integrity (KCFI) (provides forward-edge control flow protection for operating systems kerneljs)
  leak-sanitizer                 Run Leak sanitizer (run-time memory leak detector)
  memory-sanitizer               Run memory sanitizer (detector of uninitialized reads)
  mem-tag-sanitizer              Run another address sanitizer (like AddressSanitizer and HardwareAddressSanitizer but with lower overhead suitable for use as hardening for production binaries)
  nightly-checks                 Run nightly-only checks through cargo-careful `<https://crates.io/crates/cargo-careful>`
  safe-stack                     Run SafeStack check (provides backward-edge control flow protection by separating stack into safe and unsafe regions)
  shadow-call-stack              Run ShadowCall check (provides backward-edge control flow protection - aarch64 only)
  thread-sanitizer               Run Thread sanitizer (data race detector)
  help                           Print this message or the help of the given subcommand(s)
```

[1]: https://github.com/matklad/cargo-xtask
[2]: https://github.com/clap-rs/clap
[3]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
[4]: https://doc.rust-lang.org/book/ch11-03-test-organization.html#integration-tests
[5]: https://doc.rust-lang.org/book/ch11-03-test-organization.html#unit-tests
[6]: https://embarkstudios.github.io/cargo-deny/
[7]: https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html
[8]: https://github.com/tracel-ai/github-actions/blob/main/.github/workflows/publish-crate.yml
