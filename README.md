# Lovett
A Rusty Pi Application Framework

## Lovett is a Framework

Lovett was designed to create small graphical applications using Rust, a framebuffer, and GPIO pins on a Rasbperry PI.

### Usage

Its not yet anywhere else on the net, so if you want to use the very early pre alpha, you will need to speciy the github location in yout Cargo.toml

```toml
[dependencies]
lovett = { git = "https://github.com/kcculhwch/lovett" }
```

In your main.rs

```rs
extern crate lovett;

mod app;

use app::*;

fn main()  {
    let app = App::new();
    run_app(app);
}
```

Create a /app directory with a mod.rs

```rs
// Define App Struc // and impl and run_app
```



## Lovett is used by SilviaPiPID

## Lovett is a big work in progress and needs lots of love

* Lovett needs a test suite
* Lovett needs some more architectural work
* Lovett needs optimization
* Lovett needs people with more experience in Rust to make it better
