import asyncio
import time

from spider_rs import Website

async def main():
    print("benching spider-rs(python)...")
    website = Website("https://rsseau.fr", False)
    start = time.time()
    website.crawl()
    end = time.time()
    links = website.get_links()
    print("pages found " + str(len(links)))
    print("elasped duration " + str(end - start))

asyncio.run(main())