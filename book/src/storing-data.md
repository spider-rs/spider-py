# Storing Data

Storing data can be done to collect the raw content for a website.

This allows you to upload and download the content without UTF-8 conversion. The property only appears when
setting the second param of the `Website` class constructor to true.

```py
import asyncio
from spider_rs import Website

class Subscription:
    def __init__(self):
        print("Subscription Created...")
    def __call__(self, page):
        print(page.url + " - bytes: " + str(page.raw_content))
        # do something with page.raw_content

async def main():
    website = Website("https://choosealicense.com")
    website.crawl(Subscription(), True)
```
