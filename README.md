# spider-py

The [spider](https://github.com/spider-rs/spider) project ported to Python.

Test url: `https://espn.com`

| `libraries`                    | `pages`   | `speed` |
| :----------------------------- | :-------- | :------ |
| **`spider-rs(python): crawl`** | `150,387` | `186s`  |
| **`scrapy(python): crawl`**    | `49,598`  | `1h`    |

The benches above were ran on a mac m1, spider on linux arm machines performs about 2-10x faster.

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

View [bench](./bench/) to see the results.

## Issues

Please submit a Github issue for any issues found.
