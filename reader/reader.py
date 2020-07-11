from time import sleep
from os import makedirs
from shutil import rmtree
from os.path import exists
from json import loads as jsonParse
from urllib.request import urlopen, urlretrieve

target = ["r/dankmemes", "r/memes"]
limit = 20

def getImages(target, limit):
    if exists("temp"):
        rmtree("temp")
    makedirs("temp")

    print("Using target subs: " + " | ".join(target))
    for subreddit in target:
        # Sleep for 2 seconds to prevent reddit blocking us with error code 429
        sleep(2)

        print("Reading target sub: " + subreddit)
        with urlopen(f"https://reddit.com/{subreddit}/top/.json?count={limit}") as url:
            data = jsonParse(url.read().decode())
            for post in data["data"]["children"]:
                url = post["data"]["url"]
                # Ignore gifs
                if not (url.endswith(".gif") or url.endswith(".gifv")):
                    print("Found post: " + url)
                    urlretrieve(url, "temp/" + url.split("/")[-1])
                    print("Successfully saved post to: temp/" + url.split("/")[-1])
                    print()


getImages(target, limit)