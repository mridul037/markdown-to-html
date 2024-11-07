### Markdown-to-HTML converter 

#### WebAssembly with Rust
Install wasm-pack if you haven’t already. This tool compiles Rust code to WebAssembly
```
cargo install wasm-pack
wasm-pack build --target web

```


`Cargo.toml` file dependecies.


```
pulldown-cmark = "0.9"
wasm-bindgen = "0.2"
```

serve using

```
npm install -g http-server
http-server .
```
