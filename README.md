# spider-py

The [spider](https://github.com/spider-rs/spider) project ported to Python.

## Getting Started

1. `pip install spider_rs`

```python
import asyncio

from spider_rs import crawl

async def main():
    website = await crawl("https://choosealicense.com")
    print(website.links)
    # print(website.pages)

asyncio.run(main())
```

Use the Website class to build the crawler you need.

```python
import asyncio

from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com", False)
    website.crawl()
    print(website.get_links())

asyncio.run(main())
```

## Development

Install maturin `pipx install maturin` and python.

1. `maturin develop`

## Benchmarks

View [bench](./bench/) to see the results. The library should run faster than scrappy by at least 2x for normal workflows and up to 100,000x for large websites.
The speed increases drastically as we harness isolated concurrency from Rust.

## Todo

1. Fix http headers custom assign.
1. Add better docs.
1. Fix benchmarks.

Once these items are done the base of the module should be complete. Most of the code comes from the initial port to Node.js that was done.
