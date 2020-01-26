<div align="center">

  <h1><code>wasm-pack-template</code></h1>

  <strong>A template for kick starting a Rust and WebAssembly project using <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a>.</strong>

  <p>
    <a href="https://travis-ci.org/rustwasm/wasm-pack-template"><img src="https://img.shields.io/travis/rustwasm/wasm-pack-template.svg?style=flat-square" alt="Build Status" /></a>
  </p>

  <h3>
    <a href="https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html">Tutorial</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/442252698964721669/443151097398296587">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## Chris

```cargo generate --git https://github.com/rustwasm/wasm-pack-template
cargo install cargo-wasm
cargo wasm setup


   Compiling bouncingball v0.1.0 (/Users/cwilkes/Documents/workspace/bouncingball)
error[E0401]: can't use generic parameters from outer function
  --> src/lib.rs:57:31
   |
56 | impl Universe {
   | ---- `Self` type implicitly declared here, by this `impl`
57 |     pub fn advance(self: &mut Self) -> Universe {
   |                               ^^^^
   |                               |
   |                               use of generic parameter from outer function
   |                               use a type here instead

error[E0277]: the trait bound `Universe: wasm_bindgen::convert::IntoWasmAbi` is not satisfied
  --> src/lib.rs:55:1
   |
55 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `wasm_bindgen::convert::IntoWasmAbi` is not implemented for `Universe`
   |
   = note: required because of the requirements on the impl of `wasm_bindgen::convert::ReturnWasmAbi` for `Universe`

error[E0277]: the trait bound `Universe: wasm_bindgen::convert::IntoWasmAbi` is not satisfied
  --> src/lib.rs:55:1
   |
55 | #[wasm_bindgen]
   | ^^^^^^^^^^^^^^^ the trait `wasm_bindgen::convert::IntoWasmAbi` is not implemented for `Universe`
   |
   = note: required because of the requirements on the impl of `wasm_bindgen::convert::ReturnWasmAbi` for `Universe`

error: aborting due to 3 previous errors


```

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

[Learn more about `cargo generate` here.](https://github.com/ashleygwilliams/cargo-generate)

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
