import asyncio
import time
from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com", False)
    a = time.time()
    website.crawl()
    print(website.get_links())
    print("time " + str((time.time() - a)))

asyncio.run(main())