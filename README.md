# bencho

A command-line benchmarking tool which is used to visualize speed comparison between commands, based on hyperfine.


## Usage

### Basic benchmarks

To run a benchmark between commands, you can simply call `benco <command> <command>...`. The argument(s) can be any
shell command. For example:
```sh
bencho 'echo 1' 'echo 2'
```

This will generate following asset:
![benco results](assets/benchmarks-plot.png)


It also support  `--prepare` and `--cleanup` args as hyperfine support.
