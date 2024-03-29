## urbit-monitor
Health monitor for urbit ships, with sms and urbit group alerting.

  - `urbit-monitor` is a simple rust-based program to monitor deployed urbit ships and alert via text or email upon issues.
  - Sends a text message using [textbelt](https://textbelt.com/). Textbelt has a free key for 1x day, otherwise is $5 for 200 texts.
  - Uses the [urbit rust API](https://github.com/robkorn/rust-urbit-http-api) developed by [dcspark](https://www.dcspark.io/) to test via ship login

### Download instructions (*nix)
Download and run release binaries using the following instructions.
#### linux
```shell
curl -LJO https://github.com/riflechess/urbit-monitor/releases/download/v0.0.2-x86_64-linux-urbit-monitor/x86_64-linux-urbit-monitor-v0.0.2.tar.gz
tar -xzf x86_64-linux-urbit-monitor-v0.0.2.tar.gz

```
#### mac m1
```shell
curl -LJO https://github.com/riflechess/urbit-monitor/releases/download/v0.0.2-arm64-m1-urbit-monitor/arm64-m1-urbit-monitor-v0.0.2.tgz
tar -xzf arm64-m1-urbit-monitor-v0.0.2.tgz

```

### Build instructions (*nix)
Build from source code using the following instructions
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
    - `alert_snooze`- After alert, don't send alerts for X cycles.
    - _example: interval: 300, alert_snooze: 6, will check status of planets every five minutes and upon alerting, halt futher alerts for 30 min.
  - `text_alerting` - (optional) comment or completely remove section if you don't want text alerts
    - `endpoint: https://textbelt.com/text` - don't change this.
    - `phone_number` - phone number of person to receive sms/text if planet goes down (e.g. 5558675309)
    - `token` - API token from textbelt. `textbelt` will give you 1 text/day, otherwise $5 for 200 texts.
    - `alert_text` - alert text. failing planets will be listed after this. (e.g.`ALERT:Check planets: `)
  - `urbit_group_alert` - (optional) comment or completely remove section if you don't want alerts to a urbit group
    - `reporter_ship_name` - ship to login to and alert to a group (e.g. `~watsup-watsup-watsup-watsup`)
    - `reporter_ship_address` - address of "reporter" ship (e.g. `https://moon.watsup.net:443`)
    - `reporter_ship_code` - `+code` for login of reporter ship (e.g. `watsup-watsup-watsup-watsup`)
    - `reporter_group_name` - chat name of to report to (grab from URL, e.g. `mychat-1313`)
    - `reporter_alert_text` - alert text. failing planets will be listed after this. (e.g.`ALERT:Check planets: `)

  - `endpoints`
    - ship name (galaxy, star, planet, moon, comet)
    - `address` accessable login endpoint, e.g. `https://sampal-palnet.net/`
    - `code` login `+code`

### Further development
  - Use `alerting_receiver` to add support for other alerting methods.
  - More intrusive tests beyond login(?)
  - More error handling

### Sample
#### Running
Run the monitor in the background and have it write to a log:
```shell
./urbit-monitor config.yml >> urbitmon.log &
```
Stop the monitor:
```shell
pkill urbit-monitor
```

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
![](img/sms-alert.jpg?raw=true)
#### Urbit Group Alert
![](img/group-alert.png?raw=true)
