# JSON Transformations - Native vs VRL vs Rhai 

> Note: benchmarks are ran within GitHub CI which might introduce a bit of noise, though, we can make a bold assumption that noise that will affect one will affect others. Since the benchmark only lasts a few seconds each.
        

| Benchmark | Time (in µs) |
|-----------|------|
| Native | 210.2 |
| VRL | 539.6 |
| Rhai | 4148.8 |

![Benchmark Bar Chart](https://quickchart.io/chart?bkg=white&c=%7B%22data%22%3A%7B%22datasets%22%3A%5B%7B%22data%22%3A%5B210.2%2C539.6%2C4148.8%5D%2C%22label%22%3A%22Benchmark%20Results%22%7D%5D%2C%22labels%22%3A%5B%22Native%22%2C%22VRL%22%2C%22Rhai%22%5D%7D%2C%22options%22%3A%7B%22scales%22%3A%7B%22yAxes%22%3A%5B%7B%22ticks%22%3A%7B%22beginAtZero%22%3Atrue%7D%7D%5D%7D%2C%22title%22%3A%7B%22display%22%3Atrue%2C%22text%22%3A%22Lower%20is%20Better%22%7D%7D%2C%22type%22%3A%22bar%22%7D)

<details><summary>Click to expand logs</summary>

Rust Benchmark Output:

```shell

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

Rhai                    time:   [4.1447 µs 4.1488 µs 4.1542 µs]
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe

VRL                     time:   [538.67 ns 539.61 ns 540.77 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild

Native                  time:   [208.42 ns 210.21 ns 212.46 ns]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe


```



</details>
