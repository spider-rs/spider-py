# Simple Example

We use the [pyo3](https://pyo3.rs/v0.20.0/) to port the Rust project to target Python.

There are some performance drawbacks from the addon, even still the crawls are lightning fast and efficient.

## Usage

The examples below can help get started with spider.

### Basic

```python
import asyncio

from spider_rs import Website

async def main():
    website = Website("https://jeffmendez.com")
    website.crawl()
    print(website.get_links())

asyncio.run(main())
```

### Events

You can pass an object that could be async as param to `crawl` and `scrape`.

```py
import asyncio
from spider_rs import Website

class Subscription:
    def __init__(self):
        print("Subscription Created...")
    def __call__(self, page):
        print(page.url + " - status: " + str(page.status_code))

async def main():
    website = Website("https://choosealicense.com")
    website.crawl(Subscription())

asyncio.run(main())
```

### Selector

The `title` method allows you to extract the title of the page.

```py
import asyncio
from spider_rs import Website

class Subscription:
    def __init__(self):
        print("Subscription Created...")
    def __call__(self, page):
        print(page.url + " - title: " + str(page.title()))

async def main():
    website = Website("https://choosealicense.com")
    website.crawl(Subscription())
```
