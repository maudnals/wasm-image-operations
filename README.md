# Wasm-image-operations    

JS vs Wasm benchmark, when perform the same basic operations on an image's pixels.

## Demo  

<p align="center">
<img with="540" src="https://github.com/maudnals/wasm-image-operations/blob/master/doc/demo.gif?raw=true"/>
</p>  

* The average pixel value (R+G+B) of the image is computed with WebAssembly. 
* A benchmark of JS vs Wasm duration to perform the operations is performed (over multiple iterations) -  see below the image.

## Run

1. Install the dependencies:

   \$ npm install

2. Start the demo:

   \$(npm bin)/parcel index.html
   parcel serve index.html --public-url /

3. Visit `http://localhost:1234` with your browser.

## Built with Parcel+Rust

Boilerplate used: https://github.com/koute/stdweb/tree/master/examples/hasher-parcel

## How does this work?

A [parcel plugin] is used to integrate `cargo-web` with Parcel.

[parcel plugin]: https://github.com/koute/parcel-plugin-cargo-web

Source: https://github.com/koute/stdweb/tree/master/examples/hasher-parcel
