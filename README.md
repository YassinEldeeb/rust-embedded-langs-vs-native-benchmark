# JSON Transformations - Native vs VRL 

| Benchmark | Time (in µs) |
|-----------|------|


![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

vrl                     time:   [620.80 ns 626.08 ns 633.01 ns]
                        change: [+0.4521% +1.3625% +2.2852%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 12 outliers among 100 measurements (12.00%)
  9 (9.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

native_rust             time:   [324.60 ns 327.92 ns 331.93 ns]
                        change: [-1.2061% +0.1084% +1.5104%] (p = 0.88 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe


```



</details>
