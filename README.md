# Carcar
Car-liked benching data generator for fog emulator.

## How to use this?

### Installation

```bash
git clone https://github.com/tz70s/carcar
cd carcar
# Install the release
cargo install
```

### Execution and Options

#### Benching Data Generator

```bash
carcar fire <options>
```

* `-h` See the detail.
* `-d` The target destination of ip:port address.
* `-c <level>` The level of concurrency.
* `-m <model file>` The file of the model.

#### Debug Server

```bash
carcar debug <options>
```

* `-t <time>` Specify keeping time of the debug server, default is 10 secs.
* `-c <level>` The level of concurrency.
* `-p <port>` The port which the debug server listened on, default is 10023.

#### List Existed Models

```bash
carcar list
```
