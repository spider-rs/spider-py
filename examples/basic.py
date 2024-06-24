import asyncio
from typing import List
from spider_rs import crawl, Website

async def main() -> None:
    website: Website = await crawl("https://choosealicense.com")
    print(website.links)

asyncio.run(main())
