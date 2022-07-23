# urbitmon
Health monitor for urbit ships. I'm sure there will be a hoon app soon, but in the meantime. :-)

  - `urbitmon` is a simple rust-based program to monitor deployed urbit ships and alert via text or email upon issues.
  - Sends a text message using [textbelt](https://textbelt.com/). Textbelt has a free key for 1x day, otherwise is $5 for 200 texts.
  - Uses the [urbit rust API](https://github.com/robkorn/rust-urbit-http-api) developed by [dcspark](https://www.dcspark.io/) to test via ship login

## Build instructions (*nix)
  - Prerequistes: _git_
  - Install Rust (ensure the arch matches your target)
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
  - Clone git repo
    ```shell
    git clone git@github.com:riflechess/urbit-monitor.git
    cd urbit-monitor
    ```
  - Build release binary
    ```shell
    cargo build --release
    ```
  - Validate build
    ```shell
    ./target/release/urbit-monitor                                                                                  /tmp/urbit-monitor:17:55
2022-07-23T17:55:55 - Staring urbitmon...
USAGE: urbitmon [yaml config file]
       e.g. urbitmon config.yaml


## Config file
  - `monitoring`
    - `interval` - Check ships every X seconds. If set to `0`, urbitmon will run once and ignore `alert_snooze`.
    - `alert_snooze` - After alert, don't send alerts for X cycles.
    - _example: interval: 300, alert_snooze: 6, will check status of planets every five minutes and upon alerting, halt futher alerts for 30 min.
  - `endpoints`
    - ship name (galaxy, star, planet, moon, comet)
    - `address` accessable login endpoint, e.g. `https://sampal-palnet.net/`
    - `code` login `+code`

## Further development
  - Add support for alerting via urbit group message.
  - Use `alerting_receiver` to add support for other alerting methods.

