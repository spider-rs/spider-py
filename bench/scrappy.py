import time
import scrapy
from scrapy.spiders import CrawlSpider, Rule
from scrapy.linkextractors import LinkExtractor
from scrapy.crawler import CrawlerProcess

class MySpider(CrawlSpider):
    name = 'rsseau.fr'
    allowed_domains = ['rsseau.fr']
    start_urls = ['https://rsseau.fr']
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
start = time.time()
spider = MySpider
process.crawl(spider)
process.start()
end = time.time()
print("pages found " + str(len(spider.links)))
print("elasped duration " + str(end - start))