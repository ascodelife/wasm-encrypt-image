# wasm-encrypt-image

## What is it?
Trying to decrypt the cipher by Rust and WebAssembly on your website, and put image on canvas to show plaintext after drawing plaintext in image.  
See this [article](https://juejin.cn/post/7080528388984668197/) for details.

## Run example
```
  1. Build WebAssembly by wasm-pack

     wasm-pack build

  2. Prepare for running example (Node.js is required)

     cd examples\\simple_example && npm install

  3. Run example

     npm run serve
```