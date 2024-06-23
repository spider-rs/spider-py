import time, scrapy, sys
from scrapy.linkextractors import LinkExtractor
from scrapy.spiders import CrawlSpider, Rule
from scrapy.crawler import CrawlerProcess
from urllib.parse import urlparse
from scrapy.settings import Settings
from scrapy.http import Response
from typing import List, Type, Union, ClassVar

url: str = len(sys.argv) > 1 and str(sys.argv[1]) or "https://rsseau.fr"
host: str = urlparse(url).hostname

class MySpider(CrawlSpider):
    name: Union[str, None] = host
    allowed_domains: List[str] = [host]
    start_urls: List[str] = [url]
    links: List[str] = []
    rules: ClassVar[tuple] = (
        Rule(LinkExtractor(), callback='parse_item', follow=True),
    )

    @classmethod
    def update_settings(cls: Type['MySpider'], settings: Settings) -> None:
        super().update_settings(settings)
        settings.set("LOG_ENABLED", "false", priority="spider")

    def parse_item(self, response: Response) -> None:
        self.links.append(response.url)

print("benching scrappy(python)...")
process: CrawlerProcess = CrawlerProcess()
spider: Type[MySpider] = MySpider
start: float = time.time()
process.crawl(spider)
process.start()
end: float = time.time()
print(url, "pages found " + str(len(spider.links)), "elapsed duration " + str(end - start) + "s", sep="\n")
