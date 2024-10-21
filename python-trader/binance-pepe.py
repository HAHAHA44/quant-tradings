import time
import dateparser
import pytz
import json

from datetime import datetime
from binance.client import Client

# create a function, get PEPEUSDT per 10 seconds and print the price
def get_pepeusdt_price():
    client = Client("", "")
    ticker = client.get_symbol_ticker(symbol="PEPEUSDT")
    return ticker["price"]

# run the function every 10 seconds
while True:
    print(get_pepeusdt_price())
    time.sleep(10)
