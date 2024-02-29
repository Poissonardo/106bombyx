## 106bombyx

This project simulates the population growth of Bombyx mori (silkworm) using Python and Rust applications.

### Project Description

The project includes two applications:

* **Python application:** located in the `src` directory, it simulates population growth and uses modules for mathematical operations, error handling, and user input.
* **Rust application (bonus):** located in the `bonus` directory, it's a command-line tool performing similar simulations with user-provided parameters.

### Applications

#### Python Application

The Python application resides in the `src` directory and utilizes the following files:

* `maths.py`: Implements mathematical operations for the simulation.
* `error_handling.py`: Manages potential errors during execution.
* `user_input.py`: Handles user input for the application.

**Running the Python application:**

Simply execute the 106bombyx executable located in the root folder with `./106bombyx` and the appropriate arguments.

#### Rust Application (Bonus)

The Rust application is found in the `bonus` directory and functions as a command-line tool. It simulates population growth based on user-provided parameters through command-line arguments.

**Command-line arguments:**

* `n`: The number of individuals in the first generation.
* `k`: The growth rate (value between 1 and 4).
* `i0` and `i1`: The initial and final generations (inclusive).

**Running modes:**

* **Two arguments (n and k):** Calculates the population growth for 100 generations.
* **Three arguments (n, i0, and i1):** Calculates the population growth from generation `i0` to `i1`.

**Building and Running the Rust application:**

1. Navigate to the `bonus` directory.
2. Ensure you have **Rust** and **Cargo** installed on your system.
3. Run the following commands:

```
cargo build
cargo run -- <arguments>
```

Replace `<arguments>` with the desired command-line arguments according to the chosen mode.

**Note:**

The `Cargo.toml` file in the `bonus` directory specifies the application's dependencies, and the `Cargo.lock` file ensures consistent builds.
