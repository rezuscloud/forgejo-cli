# forgejo-cli

CLI tool for interacting with Forgejo

## Installation

### Pre-built

Pre-built binaries are available for `x86_64` Windows and Linux (GNU) on the
[releases tab](https://codeberg.org/Cyborus/forgejo-cli/releases/latest).

### From source

Install with `cargo install`

```
# Latest version
cargo install --git https://codeberg.org/Cyborus/forgejo-cli.git --tag v0.0.4
# From `main`
cargo install --git https://codeberg.org/Cyborus/forgejo-cli.git --branch main
```

### OCI Container

`forgejo-cli` is available as an OCI container for use in CI, at
`codeberg.org/cyborus/forgejo-cli:latest`

## Usage

### Instance-specific aliases

While you can just use the `fj` binary directly, it can be useful to alias it
with the `--host` flag set, to create shorthands for certain instances.

```bash
# For example, a `cb` command for interacting with codeberg
alias cb="fj --host codeberg.org"
# Or disroot
alias dr="fj --host git.disroot.org"
# Or any other instance you want!
# And the alias name can be whatever, as long as the `--host` flag is set.
```

Now, when you reference a repository such as `forgejo/forgejo`, it will
implicitly get it from whichever alias you used!

```
$ cb repo info forgejo/forgejo
forgejo/forgejo
> Beyond coding. We forge.

Primary language is Go
# etc...
```

When using `fj` directly, you'd have to use a URL to access it.

```
$ fj repo info codeberg.org/forgejo/forgejo
forgejo/forgejo
> Beyond coding. We forge.

Primary language is Go
# etc...

# Notice the "dr", trying to access Disroot, still works when you specify Codeberg in the repository name!
$ dr repo info codeberg.org/forgejo/forgejo
forgejo/forgejo
> Beyond coding. We forge.

Primary language is Go
# etc...
```

## Licensing

This project is licensed under either
[Apache License Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

