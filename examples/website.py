import asyncio
import time
from typing import List
from spider_rs import Website

async def main() -> None:
    website: Website = Website("https://choosealicense.com", False)
    start_time: float = time.time()
    website.crawl()
    links: List[str] = website.get_links()
    print(links)
    print(f"time {time.time() - start_time}")

asyncio.run(main())
