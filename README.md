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
    website = Website("https://choosealicense.com", False).with_headers({ "authorization": "myjwttoken" })
    website.crawl()
    print(website.get_links())

asyncio.run(main())
```

Setting up real time subscriptions can be done too.


```python
import asyncio

from spider_rs import Website

class Subscription:
    def __init__(self): 
        print("Subscription Created...") 
    def __call__(self, page): 
        print(page.url + " - status: " + str(page.status_code))

async def main():
    website = Website("https://choosealicense.com", False)
    website.crawl(Subscription())

asyncio.run(main())
```

View the [examples](./examples/) for more.

## Development

Install maturin `pipx install maturin` and python.

1. `maturin develop`

## Benchmarks

View [bench](./bench/) to see the results.

## Issues

Please submit a Github issue for any issues found.