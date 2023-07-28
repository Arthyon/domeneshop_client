# Domeneshop Client

This crate contains bindings for the public API to Domeneshop's services.
API documentation is located [here](https://api.domeneshop.no/docs).

## Prerequisites

To use this crate, you need to generate API-credentials for the Domeneshop API.
See their [Authentication](https://api.domeneshop.no/docs/#section/Authentication)-section for more information.

## Usage

To add this crate to your project, add a reference in your `Cargo.toml` to this github repository:

```toml
domeneshop_client = { git = "https://github.com/Arthyon/domeneshop_client.git" }
```

### Creating the client

```rust
use domeneshop_client::client::{ DomeneshopClient, DomeneshopClientConfiguration };

let client = DomeneshopClient::new(token, secret, DomeneshopClientConfiguration::default());
```

## Disclaimer

I am not a professional rust-developer, so this crate may contain non-idiomatic and inefficient code.
All feedback and contributions are welcome to improve the quality!

## License

Licensed under MIT license ([LICENSE](LICENSE)).
