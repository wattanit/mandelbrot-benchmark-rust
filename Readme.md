# Manbelbrot Benchmark (Rust)

## How to run the benchmark

1. Build docker image
```bash
docker build -t mandelbrot-benchmark-rust . 
```

2. Run the benchmark script
- to run in a single core mode
```bash
time ./run_benchmark.sh <image size>
```
- to run in a multi core mode
```bash
time ./run_benchmark.sh <image size> parallel
```

## Benchmark results

### Macbook Pro 2017 (2.3 GHz Intel Core i5)
| image size | single core time (s) | multi core time (s) |
|------------|----------------------|---------------------|
| 501x501    | 2.141                | 1.297               |
| 1001 x 1001| 3.028                | 2.915               |
| 2001 x 2001| 10.138               | 9.089               |
| 3001 x 3001| 21.830               | 20.946              |

### Macbook Air 2020 (M1)
| image size | single core time (s) | multi core time (s) |
|------------|----------------------|---------------------|
| 501x501    |                      |                     |
| 1001 x 1001|                      |                     |
| 2001 x 2001|                      |                     |
| 3001 x 3001|                      |                     |

### Macbook Pro (2021 M1 Pro)
| image size | single core time (s) | multi core time (s) |
|------------|----------------------|---------------------|
| 501x501    |                      |                     |
| 1001 x 1001|                      |                     |
| 2001 x 2001|                      |                     |
| 3001 x 3001|                      |                     |

### Macbook Air (2022 M2)
| image size | single core time (s) | multi core time (s) |
|------------|----------------------|---------------------|
| 501x501    |                      |                     |
| 1001 x 1001|                      |                     |
| 2001 x 2001|                      |                     |
| 3001 x 3001|                      |                     |

