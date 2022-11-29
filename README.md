# WASM Tutorial

In this tutorial, we will build a frontend application in purely Rust: a counter which you can increment and decrement with buttons.

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed, this tutorial uses Rust to compile WebAssembly binary.

## Step 1: Add Dependency to TOML

Seed is a frontend Rust framework for creating fast and reliable web apps with an elm-like architecture.

```
[dependencies]
seed = "0.9"
```

Note: Seed is probabaly not a production level library, the power of Rust lies in its performance power not styling.

## Step 2: Create an alert

Blob import seed crate to `main.go`

```
use seed::{prelude::*, *}
```

To test that it works, we will create a pop up alert window through the library:

```
window().alert_with_message("Hello, World");
```

## Step 3: Allow WASM compilation

```
rustup target add wasm32-unknown-unknown
```

First unknown is the system that you are compiling on, and the second is the system you are targeting.
This is telling rust compiler to compile on almost any machine, run on almost any machine.

## Step 4: Serve

Trunk is a WASM web application bundler for Rust.

Install Trunk

```
cargo install trunk
```

Create source HTML file `index.html` for Trunk, then run

```
trunk serve
```
