import time
import dateparser
import pytz
import json

from datetime import datetime
from binance.client import Client

with open("config.json", "r") as f:
    config = json.load(f)

# show my current balance and all informations and my selling and buying tickets
def show_my_balance():
    client = Client(config["api_key"], config["api_secret"])
    balance = client.get_account()

    # 获取我的所有订单
    orders = client.get_all_orders(symbol="PEPEUSDT", startTime=int((datetime.now().timestamp() - 60 * 60 * 24 * 5)*1000))
    return balance, orders

a, b = show_my_balance()
print(b)

print(int((datetime.now().timestamp() - 60 * 60 * 24 * 30)*1000))
