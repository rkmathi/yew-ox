# yew-ox

## Build

```bash
$ cargo install wasm-pack

$ cd /path/to/yew-ox
$ wasm-pack build --target web --out-name wasm --out-dir ./static
```

## Run with [http-server](https://www.npmjs.com/package/http-server)

```bash
$ cd /path/to/yew-ox
$ http-server ./static
#=> Open http://127.0.0.1:8080
```
