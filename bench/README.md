# Benches

You can run the benches with python in terminal.

```sh
python scrapy.py && python spider.py
```

## Cases

```
mac Apple M1 Max
10-core CPU
64 GB of RAM memory
```

URL used `https://rsseau.fr`

[Scrapy](scrapy.py)

```
Scrapy
pages found 188
elasped duration 9.301506042480469
```

[Spider-RS](spider.py)

```
Spider
pages found 200
elasped duration 5.860108852386475
```

Linux performance for Spider-RS increases around 10x especially on Arm.