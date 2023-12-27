# Scrape

Scape a website and collect the resource data.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com")
    website.scrape()
    print(website.get_pages())
    # [ { url: "https://rsseau.fr/blog", html: "<html>...</html>"}, ...]

asyncio.run(main())
```

## Headless Chrome

Headless Chrome rendering can be done by setting the third param in `crawl` or `scrape` to `true`.
It will attempt to connect to chrome running remotely if the `CHROME_URL` env variable is set with chrome launching as a fallback. Using a remote connection with `CHROME_URL` will
drastically speed up runs.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com")
    website.scrape(NULL, NULL, True)
    print(website.get_pages())
    # [ { url: "https://rsseau.fr/blog", html: "<html>...</html>"}, ...]

asyncio.run(main())
```
