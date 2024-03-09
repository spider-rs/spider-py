# spider-py

The [spider](https://github.com/spider-rs/spider) project ported to Python.

## Getting Started

1. `pip install spider_rs`


```python
import asyncio

from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com")
    website.crawl()
    print(website.get_links())

asyncio.run(main())
```

View the [examples](./examples/) to learn more.

## Development

Install maturin `pipx install maturin` and python.

1. `maturin develop`

## Benchmarks

View the [benchmarks](./bench/README.md) to see a breakdown between libs and platforms.

Test url: `https://espn.com`

| `libraries`                  | `pages`   | `speed` |
| :--------------------------- | :-------- | :------ |
| **`spider(rust): crawl`**    | `150,387` | `1m`    |
| **`spider(nodejs): crawl`**  | `150,387` | `153s`  |
| **`spider(python): crawl`**  | `150,387` | `186s`  |
| **`scrapy(python): crawl`**  | `49,598`  | `1h`    |
| **`crawlee(nodejs): crawl`** | `18,779`  | `30m`   |

The benches above were ran on a mac m1, spider on linux arm machines performs about 2-10x faster.

## Issues

Please submit a Github issue for any issues found.
