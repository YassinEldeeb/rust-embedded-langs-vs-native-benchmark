# JSON Transformations - Native vs VRL 

| Benchmark | Time (in µs) |
|-----------|------|
| Rust - native_rust | 233.73 |
| Rust - vrl | 524.42 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B233.73%2C524.42%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Rust%20-%20native_rust%22%2C%22Rust%20-%20vrl%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

vrl                     time:   [524.22 ns 524.42 ns 524.63 ns]
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) low severe
  5 (5.00%) high mild
  1 (1.00%) high severe

native_rust             time:   [233.53 ns 233.73 ns 233.93 ns]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild


```



</details>
