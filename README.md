# Gear helper library for dApps

[![Docs][docs_badge]][docs_href]
[![Actions][actions_badge]][actions_url]
[![License][lic_badge]][lic_href]

[docs_badge]: https://img.shields.io/badge/Docs-online-5023dd
[docs_href]: https://dapp.rs/gear-lib

[actions_badge]: https://img.shields.io/github/actions/workflow/status/gear-dapps/gear-lib/ci.yml?label=CI
[actions_url]: https://github.com/gear-dapps/gear-lib/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: LICENSE

This library contains basic implementations of:
- fungible token
- non fungible token
- multi token
- their encodable equivalents

and the Transaction Manager.

To add the library in your dApp, include this line under the `[dependencies]` section in `Cargo.toml`:
```toml
gear-lib = { git = "https://github.com/gear-dapps/gear-lib", tag = "0.4.1" }
```

## License

The source code is licensed under the [MIT license](LICENSE).
