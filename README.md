# rust-lang-ar.github.io

This is the source code for Rust's Argentina GitHub Page

## Development

The website in written using Rust's Yew framework, a front-end framework for
creating dynamic web applications.

Yew communicates with JavaScript using the WASM Bindgen, all the Rust files
inside of the `src` directory will be compiled to a single WASM file.

Webpack is used to build such bundle contaning:

1. The WASM build
2. A JavaScript file in charge of loading WASM into the browser.
3. All the CSS in bundled into the output `website.js`
4. Copies the HTML file located in the `static` directory to the `dist` directory

### Requirements

- NodeJS
- Rust

#### Why do I need NodeJS?

NodeJS is used to run `yarn` which is a package manager in NodeJS-land.
With `yarn` we install webpack which is our bundler for this project and makes
the bundle job we can check on the `webpack.config.js` file.

#### Why do I need Rust?

Rust is required to compile Yew framework's source code. Yew is written in Rust
and compiled to WASM.

### Scripts

> In order to execute the following commands, NodeJS and `yarn` must be installed

Script | Description
--- | ---
`yarn dev` | Runs WebpackJS server on development mode
`yarn build` | Builds Rust into WASM and creates a bundle on `dist/` directory

## Contributing

All contributions to this project are welcome! Feel free to open a Pull Request
or Issue.

## License

This project is licensed under both The MIT License and The Apache License
version 2.

Refer to both licenses here:

- [MIT](./LICENSE-MIT)
- [APACHE](./LICENSE-APACHE)
