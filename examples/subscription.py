import asyncio
import time
from typing import Any
from spider_rs import Website

class Subscription:
    def __init__(self) -> None:
        print("Subscription Created...")
    
    def __call__(self, page: Any) -> None:
        print(f"{page.url} - status: {page.status_code}")

async def main() -> None:
    website: Website = Website("https://www.drake.com").with_budget({"*": 200})
    start_time: float = time.time()
    website.crawl(Subscription())
    print(f"time {time.time() - start_time}")

asyncio.run(main())
