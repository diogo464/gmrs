# Gmrs
Rust library to create garry's mod binary modules.

# Usage
Add this to your `Cargo.toml`.
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
gmrs = { git = "https://github.com/diogo464/gmrs" }
```
Compile using the `i686-unknown-linux-gnu` target, 64-bit might work as well but I couldn't get event get a c++ module working without crashing so rust probably wont work either.  
Never tested on windows/osx.  
If you get the error `Couldn't load module library!` while loading the module in garry's mod try compiling with an older version of GLIBC, using a container running ubuntu 18.04 will probably work.

# Quick start
```rust
use gmrs::prelude::*;

#[gmrs::function]
fn hello_world(state: LuaStateRaw) -> lua::Result<()> {
    gmrs::print!(state, "Hello from rust");
    Ok(())
}

#[gmrs::entry]
fn main(state: LuaStateRaw) {
    gmrs::set_global(state, "rust_hello_world", NativeFunc::new(hello_world));
}

#[gmrs::exit]
fn exit(_state: LuaStateRaw) {}

```