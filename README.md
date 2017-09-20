# Carcar
Benching data generator for fog emulator.

* unstable currently

## Execution
```bash
# The car is the bench, fake is the testable sync tcp socket server
# The total payloads is number_of_rounds * number_of_threads
cargo run [car/fake] [number_of_rounds] [number_of_threads]

# if required infinte round, pass number_of_rounds as 0
cargo run [car/fake] 0 [number_of_threads]
```

