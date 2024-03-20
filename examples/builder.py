import asyncio

from spider_rs import Website

async def main():
    website = (
        Website("https://choosealicense.com", False)
        .with_user_agent("BotBot")
        .with_headers({"authorization": "Something "})
    )
    website.crawl()
    print(website.get_links())


asyncio.run(main())
