# JSON Transformations - Native vs VRL vs Rhai 

| Benchmark | Time (in µs) |
|-----------|------|
| Native | 392.7 |
| VRL | 741.6 |
| Rhai | 3802.8 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B392.7%2C741.6%2C3802.8%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Native%22%2C%22VRL%22%2C%22Rhai%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Rhai                    time:   [3.7976 µs 3.8028 µs 3.8083 µs]
                        change: [-0.5900% -0.2517% +0.0730%] (p = 0.14 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

VRL                     time:   [739.09 ns 741.59 ns 744.35 ns]
                        change: [+2.0240% +2.6263% +3.2518%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

Native                  time:   [390.69 ns 392.70 ns 394.65 ns]


```



</details>
