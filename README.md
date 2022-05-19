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

## Gitpod integration

[Gitpod](https://www.gitpod.io/) container-based development platform will be enabled on your project by default.

Workspace contains:

- **rust**: for builds
- [wasmd](https://github.com/CosmWasm/wasmd): for local node setup and client
- **jq**: shell JSON manipulation tool

Follow [Gitpod Getting Started](https://www.gitpod.io/docs/getting-started) and launch your workspace.
