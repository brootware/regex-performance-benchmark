# regex-performance-benchmark

A repository to demonstrate and compare regular expression performance between rust and golang for matching and replacing.

## TIY(Test-It-Yourself)

To test it out yourself, clone the repo locally as below

```bash
git clone https://github.com/brootware/regex-performance-benchmark.git && cd regex-performance-benchmark
```

To run go's binary

```bash
cd go-regex && go build -ldflags "-s -w" -o go-regex
./go-regex
```

To run rust's binary

```bash
cd rust-regex && cargo build --release
cp ../target/release/rust-regex .
./rust-regex
```
