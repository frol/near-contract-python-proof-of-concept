# Proof of Concept of using Python for NEAR Contracts

I used [RustPython](https://rustpython.github.io/) to create this proof of concept, more specifically, [this fork](https://github.com/RustPython/RustPython/pull/4875) that trims away wasm_bindgen which is not supported in NEAR environments since NEAR environment is not a browser environment.
Here is how to compile it:

```sh
cargo build --target wasm32-unknown-unknown --release
```

NEAR mainnet and testnet has a limit of 4.1MB for the [contract size and transaction size](https://github.com/near/nearcore/blob/master/utils/mainnet-res/res/mainnet_genesis.json#L196-L198), but the resulting binary I got is 9.1MB, so I tried to strip it:

```sh
wasm-opt -Oz --strip --vacuum target/wasm32-unknown-unknown/release/pycontract.wasm -o pycontract.wasm
```

I still get 4.3MB Wasm, which is not enough to push it through.
Next steps are:

1. further tweak RustPython to avoid remaining `__wbindgen_*` imports (see WAT output to see that the current contract tries to import those host methods that are not available, so it won't get executed in NEAR)
2. run a localnet (e.g. with near-workspaces) to get it tested to potentially push NEAR Protocol changes to increase the limits.
