# Beep SFU server
Beep SFU is an implementation of a SFU server for the messaging app Beep.
## Env

```.env
APP_KEY=123455QSDF // Corresponding to the app the api is using to sign jwt
VALUE_PORT_MIN=3478 // Range of port used for udp and sfu
VALUE_PORT_MAX=3495
SIGNAL_PORT=8080 // Http port to get request from clients
IP_ENDPOINT=0.0.0.0 // Prefer to use the Ip you are running your servers on 
```
To run from localhost:
```bash
export RUST_LOG=debug &&
export APP_KEY=G9e1d_eQQQpnbiEAeBa7uqYXwRgtecNL &&
export VALUE_PORT_MIN=3478 &&
export VALUE_PORT_MAX=3479 &&
export SIGNAL_PORT=8080 &&
export IP_ENDPOINT=127.0.0.1
```

## How to run it ?
### Dev mode
```
cargo run
```
**Dev mode features** :
- Logging in stdout
### Production mode (default)
```
# mkdir /var/log/beep-sfu 
# chown <your-user>:<your-group> /var/log/beep-sfu
$ cargo run
```
**Production mode features** :
- Logging in a dedicated file `/var/log/beep-sfu/beep-sfu.log<timestamp>`

## Lint the project

```bash
$ cargo clippy --all -- --D warnings
```