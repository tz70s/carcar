# Carcar
Car-liked benching data generator for fog emulator.

## How to use this?

### Installation

```bash
git clone https://github.com/tz70s/carcar
cd carcar
# build the release
cargo build --release
```

Adter installation, we can find the execution binary in `target/release/carcar`, also can add it to the path.
To remark, the execution path should reside with model directory or specify the `-m` option for model path. 

### Execution and Options

#### Benching Data Generator
```bash
./carcar <options>
```

* `-c <level>` The level of concurrency.
* `-m <model file>` The file of the model.

#### Debug Server
```bash
./carcar -d <options>
```

* `-d` Spawn a server for debug.
* `-t <time>` Specify keeping time of the debug server, default is 10 secs.
* `-c <level>` The level of concurrency.
* `-p <port>` The port which the debug server listened on, default is 10023.
