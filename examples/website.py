import asyncio

from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com", False)
    website.crawl()
    print(website.get_links())

asyncio.run(main())