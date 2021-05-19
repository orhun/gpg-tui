# Contributing

Thank you for considering contributing to [gpg-tui](https://github.com/orhun/gpg-tui/)!

When contributing, please first discuss the change you wish to make via [issue](https://github.com/orhun/gpg-tui/issues/new/choose), [discussion](https://github.com/orhun/gpg-tui/discussions/new),
[email](mailto:orhunparmaksiz@gmail.com), or any other method with the owners of this repository before making a change.

Note that we have a [Code of Conduct](./CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Development

1. Fork this repository and create your branch from `master`.

2. Clone your forked repository:

```sh
git clone https://github.com/{user}/gpg-tui && cd gpg-tui/
```

3. Make sure you have everything listed in the [requirements](./README.md#requirements) section installed. If so, build the project:
   
```sh
cargo build
```

4. Start committing your changes.

5. Add your tests (if you haven't already) or update the existing tests according to the changes. And check if the tests are passed.

```sh
cargo test
# Include other tests
cargo test --features tui-tests,gpg-tests
```

6. Make sure [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) don't show any errors.

```sh
cargo fmt --all -- --check --verbose
cargo clippy --verbose -- -D warnings
```

## Create a Pull Request

1. Ensure that you updated the documentation and filled the [Pull Request template](.github/PULL_REQUEST_TEMPLATE.md) according to the changes you made.

2. Wait for approval from the repository owners. Discuss the possible changes and update your Pull Request if necessary.

3. You may merge the Pull Request once you have the sign-off of the repository owners, or if you do not have permission to do that, you may request them to merge it in case they haven't after a while.

# License

By contributing, you agree that your contributions will be licensed under [The MIT License](./LICENSE).
