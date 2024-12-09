# env2toml

Convert env vars to toml text.

[![Latest version][crates-badge]][crates-url]
[![All downloads](https://img.shields.io/crates/d/env2toml.svg)](https://crates.io/crates/env2toml)
[![Downloads of latest version](https://img.shields.io/crates/dv/env2toml.svg)](https://crates.io/crates/env2toml)
[![MIT License][license-badge]][license-link]
[![Github Tags][tags-badge]][tags-link]
[![Github Issues][issues-badge]][issues-link]
[![Build Status][build-badge]][build-link]

[crates-url]: https://crates.io/crates/env2toml
[crates-badge]: https://img.shields.io/crates/v/env2toml.svg
[license-badge]: https://img.shields.io/github/license/mark0725/env2toml-rs
[license-link]: https://github.com/mark0725/env2toml-rs/blob/master/LICENSE
[tags-badge]: https://img.shields.io/github/tag/mark0725/env2toml-rs.svg
[tags-link]: https://github.com/mark0725/env2toml-rs/tags
[issues-badge]: https://img.shields.io/github/issues/mark0725/env2toml-rs.svg
[issues-link]: https://github.com/mark0725/env2toml-rs/issues
[build-badge]: https://github.com/mark0725/env2toml-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/mark0725/env2toml-rs/actions?query=workflow%3ARust+branch%3Amain



## Syntax
`__` split to `.`

```
APP_TITLE='TOML Example'
APP_OWNER__NAME='Tom Preston-Werner'
APP_DATABASE__ENABLED=true
APP_DATABASE__PORT=3306
APP_DATABASE__PORTS='[ 8000, 8001, 8002 ]'
APP_SERVERS__ALPHA__IP=10.0.0.1
APP_SERVERS__ALPHA__ROLE=frontend
APP_SERVERS__BETA__IP=10.0.0.2
APP_SERVERS__BETA__ROLE=backend
APP_USERS__0__NAME=USER0
APP_USERS__0__PASSWORD=u0
APP_USERS__1__NAME=USER1
APP_USERS__1__PASSWORD=u1
```
PRIFIX: `APP_`

RESULT：
```toml
title="TOML Example"
[owner]
name="Tom Preston-Werner"

[database]
enabled=true
port=3306
ports=[ 8000, 8001, 8002 ]

[servers]

[servers.alpha]
ip="10.0.0.1"
role="frontend"

[servers.beta]
ip="10.0.0.2"
role="backend"

[[users]]
name="USER0"
password="u0"

[[users]]
name="USER1"
password="u1"

```

## Usage

```rust
use dotenvy;
use env2toml::env2toml;


fn main() {
    dotenvy::dotenv().ok();
    let result = env2toml("APP_").unwrap();
    println!("\n{}", result);
}
```

## License

This project is licensed under the [MIT license](LICENSE).