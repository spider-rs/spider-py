# Website

The website class is the foundation to Spider.

## Builder 

We use the builder pattern to configure our crawler.

```python
import asyncio

from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com", False).with_headers({ "authorization": "myjwttoken" })

asyncio.run(main())
```

## Subscriptions

```python
import asyncio

from spider_rs import Website

class Subscription:
    def __init__(self): 
        print("Subscription Created...") 
    def __call__(self, page): 
        print(page.url + " - status: " + str(page.status_code)) 
        # uncomment to perform extra parsing and get the page title 
        # print(page.url + " - title: " + page.title()) 

async def main():
    website = Website("https://choosealicense.com", False)
    website.crawl(Subscription())

asyncio.run(main())
```