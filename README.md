# JSON Transformations - Native vs VRL 

| Benchmark | Time (in µs) |
|-----------|------|
| Rust - native_rust | 333.69 |
| Rust - vrl | 622.79 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B333.69%2C622.79%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Rust%20-%20native_rust%22%2C%22Rust%20-%20vrl%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

vrl                     time:   [617.23 ns 622.79 ns 628.74 ns]
                        change: [-1.0770% -0.2594% +0.5696%] (p = 0.54 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

native_rust             time:   [331.71 ns 333.69 ns 335.62 ns]
                        change: [-0.3746% +1.1183% +2.4255%] (p = 0.12 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild


```



</details>
