import asyncio
import time

from spider_rs import Website

async def main():
    website = Website("https://choosealicense.com", False)
    start = time.time()
    website.crawl()
    end = time.time()
    links = website.get_links()
    print("links found " + str(len(links)))
    print("elasped duration " + str(end - start))

asyncio.run(main())