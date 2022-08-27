## urbitmon
Health monitor for urbit ships. I'm sure there will be a hoon app soon, but in the meantime. :-)

  - `urbitmon` is a simple rust-based program to monitor deployed urbit ships and alert via text or email upon issues.
  - Sends a text message using [textbelt](https://textbelt.com/). Textbelt has a free key for 1x day, otherwise is $5 for 200 texts.
  - Uses the [urbit rust API](https://github.com/robkorn/rust-urbit-http-api) developed by [dcspark](https://www.dcspark.io/) to test via ship login

### Build instructions (*nix)
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
### Config file
  - `monitoring`
    - `interval` - Check ships every X seconds. If set to `0`, urbitmon will run once and ignore `alert_snooze`.
    - `alert_snooze` - After alert, don't send alerts for X cycles.
    - _example: interval: 300, alert_snooze: 6, will check status of planets every five minutes and upon alerting, halt futher alerts for 30 min.
  - `endpoints`
    - ship name (galaxy, star, planet, moon, comet)
    - `address` accessable login endpoint, e.g. `https://sampal-palnet.net/`
    - `code` login `+code`

### Further development
  - Use `alerting_receiver` to add support for other alerting methods.
  - More intrusive tests beyond login(?)

### Sample
#### Log
  ```shell
  % ./target/release/urbit-monitor config.yml
  2022-08-26T19:17:17 - Staring urbitmon...
  2022-08-26T19:17:17 - Checking: zod (http://127.0.0.1:8080/)
  2022-08-26T19:17:17 - zod [ERROR] Login failed.
  2022-08-26T19:17:17 - Checking: walbex-hasper-pollyt-hocnyx--macben-padper-lagdeb-marzod (https://urbit.watsup.net/)
  2022-08-26T19:17:17 - walbex-hasper-pollyt-hocnyx--macben-padper-lagdeb-marzod [OK]
  2022-08-26T19:17:17 - Checking: turnip-hodler (https://turnip-hodler.watsup.net/)
  2022-08-26T19:17:17 - turnip-hodler [OK]
  2022-08-26T19:17:17 - Checking: tictuc-lacdef-watsup-lomrem (https://moon.watsup.net/)
  2022-08-26T19:17:17 - tictuc-lacdef-watsup-lomrem [OK]
  2022-08-26T19:17:17 - Checking: pophat-buntuk (yyyyy.net)
  2022-08-26T19:17:17 - pophat-buntuk [ERROR] Login failed.
  2022-08-26T19:17:17 - Alert triggered - planets: zod pophat-buntuk  type: text_alert
  2022-08-26T19:17:17 - https://textbelt.com/text 5558675309 fix your stinking planets: zod pophat-buntuk
  2022-08-26T19:17:18 - Alert sent [OK]
  2022-08-26T19:17:18 - Alert triggered - planets: zod pophat-buntuk  type: urbit_alert
  2022-08-26T19:17:18 - Sending alert to: ~tictuc-lacdef-watsup-lomrem, chat: alerts-6170
  2022-08-26T19:17:18 - Exiting urbitmon.
```
#### SMS/Text Alert
![](img/sms-alert.png?raw=true)
#### Urbit Group Alert
![](img/group-alert.png?raw=true)
