## About

This is a demo web application for `descriptor-encrypt`, a rust library that encrypts any Bitcoin wallet descriptor such that it can only be recovered by a set of a keys that can spend the funds.

Supports all descriptor types and miniscript.

## 🚴 Usage

### 🛠️ Build with `wasm-pack build`

```
CC=/opt/homebrew/opt/llvm/bin/clang \
AR=/opt/homebrew/opt/llvm/bin/llvm-ar \
wasm-pack build --target web
```

### 🎁 Install `http-server`

```
npm install -g http-server
```

### 🎁 Run `http-server`

```
http-server
```

## License

MIT license ([LICENSE](LICENSE))
