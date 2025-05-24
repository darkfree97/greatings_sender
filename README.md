## This is an attempt to implement a Celery workers in rust
### To build the project need to run command
```shell
cargo build --release
```
In `/target/release` you will find two executables `beat` and `love_sender`:
 - *beat* - schedules the tasks to be executed
 - *love_sender* - actual Celery worker app that runs the tasks (name of the package)
