# Website

The Website class is the foundations to the spider.

## Builder pattern

We use the builder pattern to configure the website for crawling.

\*note: Replace `https://choosealicense.com` from the examples below with your website target URL.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com")
    website.crawl()
    print(website.get_links())

asyncio.run(main())
```

### Custom Headers

Add custom HTTP headers to use when crawling/scraping.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_headers({ "authorization": "mytoken"})

asyncio.run(main())
```

### Blacklist

Prevent crawling a set path, url, or pattern with Regex.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_blacklist_url(["/blog", "/resume"])

asyncio.run(main())
```

### Crons

Setup a cron job that can run at any time in the background using cron-syntax.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_cron("1/5 * * * * *")

asyncio.run(main())
```

View the [cron](./cron-job.md) section for details how to use the cron.

### Budget

Add a crawl budget that prevents crawling `x` amount of pages.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_budget({
    "*": 1,
  })

asyncio.run(main())
```

### Subdomains

Include subdomains in request.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_subdomains(True)

asyncio.run(main())
```

### TLD

Include TLDs in request.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_tlds(True)

asyncio.run(main())
```

### External Domains

Add external domains to include with the website.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_external_domains(["https://www.myotherdomain.com"])

asyncio.run(main())
```

### Proxy

Use a proxy to crawl a website.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_proxies(["https://www.myproxy.com"])

asyncio.run(main())
```

### Depth Limit

Set the depth limit for the amount of forward pages.

```ts
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_depth(3)

asyncio.run(main())
```

### Cache

Enable HTTP caching, this useful when using the spider on a server.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_caching(True)

asyncio.run(main())
```

### Delays

Add delays between pages. Defaults to none.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_delays(200)

asyncio.run(main())
```

### User-Agent

Use a custom User-Agent.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_user_agent("mybot/v1")

asyncio.run(main())
```

### Request Timeout

Add a request timeout per page in miliseconds. Example shows 30 seconds.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_request_timeout(30000)

asyncio.run(main())
```

### Respect Robots

Respect the robots.txt file.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_respect_robots_txt(True)

asyncio.run(main())
```

### Http2 Prior Knowledge

Use http2 to connect if you know the website servers supports this.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_http2_prior_knowledge(True)

asyncio.run(main())
```

## Chaining

You can chain all of the configs together for simple configuration.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com").with_subdomains(true).with_tlds(true).with_user_agent("mybot/v1").with_respect_robots_txt(true)

asyncio.run(main())
```

## Raw Content

Set the second param of the website constructor to `true` to return content without UTF-8.
This will return `rawContent` and leave `content` when using subscriptions or the Page Object.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com", True)
    website.scrape()

asyncio.run(main())
```

## Clearing Crawl Data

Use `website.clear` to remove the links visited and page data or `website.drain_links` to drain the links visited.

```py
import asyncio
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com")
    website.crawl()
    print(website.getLinks())
    website.clear()
    print(website.getLinks())

asyncio.run(main())
```

## Stop crawl

To stop a crawl you can use `website.stopCrawl(id)`, pass in the crawl id to stop a run or leave empty for all crawls to stop.

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
    # sleep for 2s and stop etc
    website.stop()

asyncio.run(main())
```
