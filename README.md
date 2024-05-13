# CapyBastr

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the tailwind css cli: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Install LLVM via Homebrew:
```bash
brew install llvm
```

Add the Homebrew-installed LLVM to your PATH environment variable:
```bash
echo 'export PATH="/usr/local/opt/llvm/bin:$PATH"' >> ~/.zshrc
```

Install dioxus cli:
```bash
cargo install dioxus-cli@0.5
```


Launch the Dioxus Fullstack app:

```bash
dx serve 
```

```bash
wasm-pack test --firefox
```