# Cron Jobs

Use a cron job that can run any time of day to gather website data.

```python
import asyncio
from spider_rs import Website

class Subscription:
    def __init__(self):
        print("Cron Created...")
    def __call__(self, page):
        print(page.url + " - status: " + str(page.status_code))

async def main():
    website = Website("https://choosealicense.com").with_cron("1/5 * * * * *").build()
    handle = await website.run_cron(Subscription());

asyncio.run(main())
```
