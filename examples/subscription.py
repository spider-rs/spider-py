import asyncio

from spider_rs import Website

class Subscription:
    def __init__(self): 
        print("Subscription Created...") 
    def __call__(self, page): 
        print(page.url + " - status: " + str(page.status_code))

async def main():
    website = Website("https://choosealicense.com", False)
    website.crawl(Subscription())

asyncio.run(main())