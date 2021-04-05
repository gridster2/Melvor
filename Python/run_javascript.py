from selenium import webdriver
from selenium.webdriver.common.desired_capabilities import DesiredCapabilities
from json import loads, dumps
from random import sample
from string import ascii_letters
from time import time, sleep
from os.path import join, isdir, abspath, dirname, isfile
from os import makedirs


def generate_identifier() -> str:
    return str(time()).replace(".", "") + "".join(sample(ascii_letters, 20))


def run_javascript(script_name: str, accept_cached: bool = False) -> dict:
    if not script_name.endswith(".js"):
        script_name += ".js"

    # identifier is used to find the right console message later
    identifier = generate_identifier()

    # find and load the relevant javascript
    js_dir = join(abspath(dirname(__file__)), "..", "Javascript")
    if not isdir(js_dir):
        raise FileNotFoundError(f"{js_dir} does not exist")

    cached_path = join(js_dir, "cached", script_name + ".cached")
    cached_dir_path = dirname(cached_path)
    if not isdir(cached_dir_path):
        makedirs(cached_dir_path)
    if accept_cached and isfile(cached_path):
        with open(cached_path, "r") as f:
            return loads(f.read())

    js_path = join(js_dir, script_name)
    with open(js_path, "r") as javascript_file:
        javascript = javascript_file.read().replace("%|%REPLACE|ME%|%", identifier)

    response = None
    # https://stackoverflow.com/a/20910684/9107360
    d = DesiredCapabilities.CHROME
    d["goog:loggingPrefs"] = {"browser": "ALL"}
    with webdriver.Chrome(desired_capabilities=d) as driver:
        driver.get("http://melvoridle.com")
        driver.set_script_timeout(999 * 999 * 999 * 999)

        # TODO: New welcome screen means not everything loads the first time - sleep
        #  for a bit while a character is created
        sleep(30)

        # wait for console-running functions to kick in
        loading_identifier = generate_identifier()
        found_loading_id = False
        while not found_loading_id:
            sleep(0.1)
            driver.execute_script(f'console.log("{loading_identifier}")')
            for entry in driver.get_log("browser"):
                if "message" in entry and loading_identifier in entry["message"]:
                    found_loading_id = True
                    break

        # then run the script
        driver.execute_script(javascript)
        while response is None:
            sleep(0.1)
            for entry in driver.get_log("browser"):
                if "message" in entry and identifier in entry["message"]:
                    response = entry["message"]
                    break
        driver.close()
    response = (
        response[response.index(identifier) + len(identifier) + 3 : -1]
        .replace(r"\"", '"')
        .replace(r'\\"', "")
    )
    response = loads(response)

    with open(cached_path, "w") as f:
        f.write(dumps(response, indent=4, sort_keys=True))
    return response


if __name__ == "__main__":
    assert " ".join(run_javascript("helloWorld.js")) == "Hello World"
