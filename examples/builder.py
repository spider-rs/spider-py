import asyncio
from typing import List
from spider_rs import Website

async def main() -> None:
    website: Website = (
        Website("https://choosealicense.com", False)
        .with_user_agent("BotBot")
        .with_headers({"authorization": "Something "})
    )
    website.crawl()
    print(website.get_links())

asyncio.run(main())
