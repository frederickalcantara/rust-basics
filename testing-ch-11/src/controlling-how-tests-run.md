# Controlling How Tests are Run

Just as `cargo run` compiles your code and then runs the resulting binary, `cargo test` compiles our code in test mode and runs the resulting test binary.

We can specify command line options to change the default behavior of `cargo test`. 

Example: The default behavior of the binary produced by `cargo test` is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results. 

Some command line options go to `cargo test`, and some go to the resulting test binary.
To separate these two types of arguments, we list the arguments that go to `cargo test` followed by the separator `--` we can use with the `cargo test`, and running the `cargo test -- --help` displays the options we can use after the separtor `--`. 


## Running Tests in Parallel or Consecutively

When we run multiple tests, by default they run in parallel using threads. 
This means the tests will finish running faster we can get feedback quicker on whether or not our code is working. 