# Page

A single page on a website, useful if you need just the root url.

## New Page

Get a new page with content.

The first param is the url, followed by if subdomains should be included, and last to include TLD's in links.

Calling `page.fetch` is needed to get the content.

```python
import asyncio
from spider_rs import Page

async def main():
    page = Page("https://choosealicense.com")
    page.fetch()

asyncio.run(main())
```

## Page Links

get all the links related to a page.

```python
import asyncio
from spider_rs import Page

async def main():
    page = Page("https://choosealicense.com")
    page.fetch()
    links = page.get_links()
    print(links)
asyncio.run(main())
```

## Page Html

Get the markup for the page or HTML.

```python
import asyncio
from spider_rs import Page

async def main():
    page = Page("https://choosealicense.com")
    page.fetch()
    links = page.get_html()
    print(links)

asyncio.run(main())
```

## Page Bytes

Get the raw bytes of a page to store the files in a database.

```python
import asyncio
from spider_rs import Page

async def main():
    page = Page("https://choosealicense.com")
    page.fetch()
    links = page.get_bytes()
    print(links)

asyncio.run(main())
```
