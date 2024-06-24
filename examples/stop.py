import asyncio
from typing import Any
from spider_rs import Website

class Subscription:
    def __init__(self) -> None:
        print("Subscription Created...")
    
    def __call__(self, page: Any) -> None:
        print(f"{page.url} - status: {page.status_code}")
        # if (website.size >= 100):
        #     website.stop()

async def main() -> None:
    website: Website = Website("https://www.drake.com")
    website.crawl(Subscription())

asyncio.run(main())
