import time, scrapy, sys
from scrapy.linkextractors import LinkExtractor
from scrapy.spiders import CrawlSpider, Rule
from scrapy.crawler import CrawlerProcess
from urllib.parse import urlparse

url = len(sys.argv) > 1 and str(sys.argv[1]) or "https://rsseau.fr"
host = urlparse(url).hostname

class MySpider(CrawlSpider):
    name = host
    allowed_domains = [host]
    start_urls = [url]
    links = []
    rules = (
        Rule(LinkExtractor(), callback='parse_item', follow=True),
    )

    @classmethod
    def update_settings(cls, settings):
        super().update_settings(settings)
        settings.set("LOG_ENABLED", "false", priority="spider")

    def parse_item(self, response):
        self.links.append(response.url)

print("benching scrappy(python)...")
process = CrawlerProcess()
spider = MySpider
start = time.time()
process.crawl(spider)
process.start()
end = time.time()
print(url, "pages found " + str(len(spider.links)), "elasped duration " + str(end - start) + "s", sep="\n")
