> ### Tarantool-leptos app.

This app is tarantool based [TodoMVC](https://github.com/tastejs/todomvc).

# How it works

> App use [tarantool](https://github.com/tarantool/tarantool) platform as database and an application server. Front-end part
of app made with [leptos](https://github.com/leptos-rs/leptos) framework with [axum](https://github.com/tokio-rs/axum) integration.

# Preparation

Install [tarantool](https://www.tarantool.io/en/download/os-installation), [tarantool-runner](https://git.picodata.io/picodata/modules/tarantool-runner), [tarantool-test](https://git.picodata.io/picodata/modules/tarantool-test), [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).

```sh
cargo install tarantool-runner
cargo install --features="bin" tarantool-test
cargo install wasm-bindgen-cli
```

# Getting started

> To run app

```sh
make run-release
```

# Testing

> To run tests

```sh
make test
```