# JSON Transformations - Native vs VRL vs Rhai 

> Note: benchmarks are ran within GitHub CI which might introduce a bit of noise, though, we can make a bold assumption that noise that will affect one will affect others. Since the benchmark only lasts a few seconds each.

| Benchmark | Time (in µs) |
|-----------|------|
| Native | 209 |
| VRL | 535.9 |
| Rhai | 4147 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B209.0%2C535.9%2C4147.0%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Native%22%2C%22VRL%22%2C%22Rhai%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Rhai                    time:   [4.1350 µs 4.1470 µs 4.1652 µs]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) low severe
  3 (3.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe

VRL                     time:   [535.63 ns 535.88 ns 536.16 ns]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

Native                  time:   [207.70 ns 209.04 ns 211.22 ns]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe


```



</details>
