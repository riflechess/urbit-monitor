# urbit-monitor
Health monitor for urbit ships.

  - `urbit-monitor` is a simple rust-based program to monitor deployed urbit ships and alert via text or email upon issues.
  - Integrates with urbit chat groups, graph store, etc.

It heavily borrows from the [urbit rust API](https://github.com/robkorn/rust-urbit-http-api) developed by [dcspark](https://www.dcspark.io/).

## Build instructions (*nix)
  - Install rust
    ```shell
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
  - Source environment
    ```shell
    source $HOME/.cargo/env
    ```
  - Check version/install
    ```shell
    rustc --version
    ```
