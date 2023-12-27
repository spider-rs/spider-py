import asyncio, time, sys
import time
from spider_rs import Website

async def main():
    print("benching spider-rs(python)...")
    url = len(sys.argv) > 1 and str(sys.argv[1]) or "https://rsseau.fr"
    website = Website(url)
    start = time.time()
    website.crawl()
    end = time.time()
    links = website.get_links()
    print(url, "pages found " + str(len(links)), "elasped duration " + str(end - start) + "s", sep="\n")

asyncio.run(main())