# Carcar
Benching data generator for fog emulator.

* unstable currently

## Execution
```bash
# The car is the bench, fake is the testable sync tcp socket server
# The total payloads is number_of_rounds * number_of_threads
cargo run [car/fake] [number_of_rounds] [number_of_threads]
```

