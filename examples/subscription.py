import asyncio
import time

from spider_rs import Website

class Subscription:
    def __init__(self): 
        print("Subscription Created...") 
    def __call__(self, page): 
        print(page.url + " - status: " + str(page.status_code))

async def main():
    website = Website("https://www.drake.com").with_budget({ "*": 200 })
    a = time.time()
    website.crawl(Subscription())
    print("time " + str((time.time() - a)))

asyncio.run(main())