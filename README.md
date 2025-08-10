# Rust vin-info for WASM

🚗 Rust [`vin-info`](https://crates.io/crates/vin-info) package compiled to WASM

<table align=center><td>

```py
vin = Vin("WP0ZZZ99ZTS392124")
print(vin.manufacturer_region())
```

<tr><td>

```js
const vin = new Vin("WP0ZZZ99ZTS392124")
console.log(vin.wmi())
```

</table>

## Installation

You can install this package using [wkg](https://github.com/bytecodealliance/wasm-pkg-tools).

```sh
wkg get --registry ghcr.io jcbhmr:vin-info@0.1.2-rc1
```

## Usage

Now you need to generate bindings for your language of choice.

TODO: Explain more.

https://github.com/DoumanAsh/vin-info

## Development

Build the project. Defaults to `wasm32-wasip2` from `.cargo/config.toml`.

```sh
cargo build
```

There's currently no automated testing. The most testing-like thing you can do is make sure that jco can generate JS bindings from the output `.wasm` file:

```sh
jco transpile target/wasm32-wasip2/debug/vin_info_wasm.wasm -o target/jco
```

This project is published to ghcr.io GitHub container registry so that it can be used with wkg.

```sh
wkg publish --registry ghcr.io --package jcbhmr:vin-info@0.1.2 target/wasm32-wasip2/release/vin_info_wasm.wasm
```

You need to already be authenticated with `docker login` though for that to work. This is best done in GitHub Actions.
