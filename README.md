# e164-phones-countries

A small utility which maps E.164 international phone numbers to ISO 3166 country
codes as well as the ISO 3166 country codes to country phone codes.

Add e164-phones-countries dependency to your project Cargo.toml file:

```json
[dependencies]
e164-phones-countries = "0.1.1"
```

Add e164-phones-countries to your project's source code:

```rust
extern crate e164-phones-countries;

use e164-phones-countries::find_iso_3166;
use e164-phones-countries::find_phone_cc;
```

Methods signatures:

```rust
fn find_iso_3166(phone:&str) -> &'static str
fn find_phone_cc(code:&str) -> &'static str
```
