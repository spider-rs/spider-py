import asyncio
from spider_rs import Website

async def main():
    website = Website("https://www.drake.com")

    class Subscription:
        def __init__(self): 
            print("Subscription Created...") 
        def __call__(self, page): 
            print(page.url + " - status: " + str(page.status_code))
            # if (website.size >= 100):
            #     website.stop()
            
    website.crawl(Subscription())

asyncio.run(main())