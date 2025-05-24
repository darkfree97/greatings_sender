## This is an attempt to implement a Celery workers in rust
### To build the project need to run command
```shell
cargo build --release
```
In `/target/release` you will find two executables `beat` and `love_sender`:
 - *beat* - schedules the tasks to be executed
 - *love_sender* - actual Celery worker app that runs the tasks (name of the package)

### Build for Raspberry Pi 3B+ (GNU/Linux aarch64)

Add linker configuration to *.cargo/config.toml*
```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```
Install platform compilers and do the compilation
```shell
sudo apt update
sudo apt install gcc-aarch64-linux-gnu
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```