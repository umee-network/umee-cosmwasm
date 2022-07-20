# Umee CosmWasm

This is an example for [cosmwasm](https://www.cosmwasm.com) smartcontracts to consume and
query [umee](https://github.com/umee-network/umee) native modules.

## Package **umee-types**

- The actual lib to connect to umee native modules is inside `packages/umee-types`
and is being used as an library inside `src` folder

## Using it in your project

- To use the **umee-types** library you can do the same as the `src/contract.rs` and
`src/msg.rs` files that consumes all the queries availables with examples

## Developing

- If you don't know that much about rust or cosmwasm, take a look at [Rust Tips](./RustTips.md)

### Compile

- To compile the local example and it's dependences packages

```shell
$~ cargo build
```

### Generating .wasm

- The `.wasm` file are the contract compiled used to deploy it in the blockchain
- To test and generate an `.wasm` file form the `src` folder

```shell
$~ cargo wasm
```

- The generated wasm file will be inside `./target/wasm32-unknown-unknown/release/*.wasm`

- To generate the `.wasm` file optimized to production run

```shell
$~ docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.4
```

- This will generate the optimized compiled contract inside the `artifact` folder

### Schemas

- Schemas (`schema` folder) can be used to understand and communicate with the smartcontract
they are basic the representation of the contract entrypoints messages and responses
structures defined in json

> tip: if you are used to solidity, it's almost the contract ABI

- To generate the schemas based on the current files changes

```shell
$~ cargo schema
```

### Publish

- Tips on how to publish rust packages, look at [Publishing](./Publishing.md)

### Add new modules

- To add new modules inside the **umee-types** package, copy the leverage query file, like

```shell
$~ cp ./packages/umee-types/src/query_leverage.rs ./packages/umee-types/src/query_${MODULE_NAME}.rs
```

- Change everything from leverage to the new module name
- Add the new enum module to the `pub enum UmeeQuery` inside `./packages/umee-types/src/query.rs`

### Add new queries

- Create the structs for params and response in the respective query module
`./packages/umee-types/src/query_${MODULE_NAME}.rs` and also a new entry in
the `pub enum UmeeQuery${MODULE_NAME}` and a new `pub const ASSIGNED_QUERY_${QUERY_NAME}`
that correspond to the same value in the `umee-network/umee/app/wasm/query/types.go` from
the umee public [repository](https://github.com/umee-network/umee)
- Add a new property that represents the query in the `pub struct StructUmeeQuery`
as an `Option<QueryParams>` because it can be nil and a new function to instantiate
the `StructUmeeQuery` struct inside `impl StructUmeeQuery` inside
`./packages/umee-types/src/query.rs`
- Expose the new query in `./packages/umee-types/src/lib.rs`

### Cargo Make

> It is expected that you have installed [cargo-make](https://github.com/sagiegurari/cargo-make)

- After your changes, you can check if everything is running as expected at onde running the following script

```shell
$~ cargo make git_flow
```

## Gitpod integration

[Gitpod](https://www.gitpod.io/) container-based development platform will be enabled on your project by default.

Workspace contains:

- **rust**: for builds
- [wasmd](https://github.com/CosmWasm/wasmd): for local node setup and client
- **jq**: shell JSON manipulation tool

Follow [Gitpod Getting Started](https://www.gitpod.io/docs/getting-started) and launch your workspace.
