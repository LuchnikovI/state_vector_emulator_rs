This is a simple parallel state-vector emulator of quantum circuits written in Rust. You can read more about its implementation in the following [blogpost](https://luchnikovi.github.io/state_vector_rs).

How to run
==========

To run numerical experiments described in the [blogpost](https://luchnikovi.github.io/state_vector_rs) you need to compile and run the code with optimizations i.e. simply run `cargo run --releas`. The program then asks you to chosse either the simulation of 5x5 2D Heisenberg model dynamics or run benchmarks based on Quantum Fourier Transform. If you choose the simulation of 5x5 2D Heisenberg model you would need to enter the parameters: a discretization step and a total number of time steps. After the program finishes the execution, you may build a gif demonstrating the dynamics by running a python scipt `plotting.py` in the `src` foldier. If you choose running benchmarks based on Quantum Fourier Transform, it saves the benchmarking results in the same folder with the binary and also displays the benchmarking results in the terminal.

The core logic of the emulator is contained in the folder `state_vector_machine` you may use it for your own numerical experiments.
