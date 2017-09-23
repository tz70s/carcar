# Carcar
Car-liked benching data generator for fog emulator.

## Execution
```bash
# The car is the bench, fake is the testable sync tcp socket server.
# The total payloads is number_of_rounds * number_of_threads.
cargo run [car] [number_of_rounds] [number_of_threads]

# If required infinte round, pass number_of_rounds as 0
cargo run [car/fake] 0 [number_of_threads]

# To run the fake-server for bench test,
# the listening port and address is equivalent to car bench destination,
# in lib.rs.
# Also, pass the number of client threads, the fake server also used to bench.
cargo run fake [num_of_client_threads]
```
