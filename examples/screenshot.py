import asyncio

from spider_rs import Website

async def main():
    website = (
        Website("https://choosealicense.com", False)
        .with_screenshot({
            "params": {
                "cdp_params": {
                    "format": None,
                    "quality": None,
                    "clip": None,
                    "from_surface": None,
                    "capture_beyond_viewport": None
                },
                "full_page": True,
                "omit_background": False
            },
            "bytes": False,
            "save": True,
            "output_dir": None
        })
    )
    website.crawl(None, None, True)
    print(website.get_links())


asyncio.run(main())
