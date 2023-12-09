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

## Todo

1. Fix http headers custom assign.
1. Add better docs.

Once these items are done the base of the module should be complete. Most of the code comes from the initial port to Node.js that was done.
