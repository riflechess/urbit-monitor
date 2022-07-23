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
    ./target/release/urbit-monitor 
    ```
    You should see a usage message with current version.
    ```shell
    USAGE: urbitmon [yaml config file]
       e.g. urbitmon config.yaml
       urbitmon: 0.0.1 
    ```
  - Distribute the binary to your monitoring server, `./target/release/urbit-monitor` and run with the `config.yml`.
    ```shell
    ./urbit-monitor config.yml
    ```
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

## Sample log
  ```
  % ./target/release/urbit-monitor config.yml
  2022-07-23T18:22:49 - Staring urbitmon...
  2022-07-23T18:22:49 - Checking: zod (http://127.0.0.1:8080/)
  2022-07-23T18:22:50 - zod [OK]
  2022-07-23T18:22:50 - Checking: walbex-hasper-pollyt-hocnyx--macben-padper-lagdeb-marzod (https://mark.sitsev.net/)
  2022-07-23T18:22:50 - walbex-hasper-pollyt-hocnyx--macben-padper-lagdeb-marzod [OK]
  2022-07-23T18:22:50 - Checking: palnet-palnet (yyyyy.net)
  2022-07-23T18:22:50 - palnet-palnet [ERROR] Login failed.
  2022-07-23T18:22:50 - Checking: pophat-buntuk (yyyyy.net)
  2022-07-23T18:22:50 - masnut-masnut [ERROR] Login failed.
  2022-07-23T18:22:50 - Alert triggered - planets: palnet-palnet masnut-masnut  type: text_alert
  2022-07-23T18:22:50 - https://textbelt.com/text 5555555555 fix your stinking planets: palnet-palnet masnut-masnut
  2022-07-23T18:22:51 - Alert sent [OK]
  ```