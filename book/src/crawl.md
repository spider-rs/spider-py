# Crawl

Crawl a website concurrently.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://rsseau.fr")
    website.crawl()
    print(website.get_links())

asyncio.run(main())
```

## Async Event

You can pass in a async function as the first param to the crawl function for realtime updates streamed.

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

## Background

You can run the request in the background and receive events with the second param set to `true`.

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
    website.crawl(Subscription(), True)
    # this will run instantly as the crawl is in the background

asyncio.run(main())
```

## Subscriptions

You can setup many subscriptions to run events when a crawl happens.

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
    website.crawl()
    subscription_id = website.subscribe(Subscription());
    website.crawl()
    website.unsubscribe(subscription_id);

asyncio.run(main())
```

## Headless Chrome

Headless Chrome rendering can be done by setting the third param in `crawl` or `scrape` to `true`.
It will attempt to connect to chrome running remotely if the `CHROME_URL` env variable is set with chrome launching as a fallback. Using a remote connection with `CHROME_URL` will
drastically speed up runs.

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
    website.crawl(Subscription(), false, True)

asyncio.run(main())
```
