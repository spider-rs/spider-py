import asyncio, time, sys
from typing import List
from spider_rs import Website

async def main() -> None:
    print("benching spider-rs(python)...")
    url: str = len(sys.argv) > 1 and str(sys.argv[1]) or "https://rsseau.fr"
    website: Website = Website(url)
    start: float = time.time()
    website.crawl()
    end: float = time.time()
    links: List[str] = website.get_links()
    print(url, "pages found " + str(len(links)), "elapsed duration " + str(end - start) + "s", sep="\n")

asyncio.run(main())
