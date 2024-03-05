# rust-lang-ar.github.io

This is the source code for Rust's Argentina GitHub Page

### Requirements

- Rust
- NodeJS

This solution makes use of Trunk for development and bundling you must
install `trunk` in your system first.

Follow the documentation available in the official [Trunk](https://github.com/thedodd/trunk)
repository.

Then make sure you have the WASM target installed otherwise
add it to your Rust targets using:

```bash
rustup target add wasm32-unknown-unknown
```

Finally give a try on the build by running one of the following commands:


```bash
# If you are a Yarn user
yarn install && yarn build
```

```bash
# If you are a NPM user (Just installed NodeJS)

# Its important to note that this project relies on Yarn
# to lock dependencies

npm install && npm run build
```

```bash
# Or you can use NPX instead
npx tailwindcss -o src/styles/tailwind.css && trunk build
```

## Development

#### Why do I need NodeJS?

NodeJS is used to have TailwindCSS support in this project. You must
install it to use the generated stylesheet and follow the project style
configurations.

#### Why do I need Rust?

Rust is required to compile Yew framework's source code. Yew is written in Rust
and compiled to WASM.

### Scripts

> In order to execute the following commands, NodeJS and `yarn` must be installed

Script | Description
--- | ---
`yarn build` | Builds TailwindCSS styles and bundles the website under `dist/` directory
`yarn dev` | Builds TailwindCSS styles and executes Trunk's development server

## Contributing

All contributions to this project are welcome! Feel free to open a Pull Request
or Issue.

## License

This project is licensed under both The MIT License and The Apache License
version 2.

Refer to both licenses here:

- [MIT](./LICENSE-MIT)
- [APACHE](./LICENSE-APACHE)
