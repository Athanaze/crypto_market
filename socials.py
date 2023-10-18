import hashlib
import json
import os
import time

def timeFolder():
    return str(int( time.time() ))+"/"
URLS = [
    "https://nitter.net/Polkadot",
    "https://teddit.net/r/dot/",
    "https://nitter.net/gavofyork",
    "https://nitter.net/Ethereum",
    "https://teddit.net/r/ethereum/",
    "https://teddit.net/r/ethfinance/",
    "https://teddit.net/r/ethtrader",
    "https://teddit.net/r/EtherMining",
    "https://teddit.net/r/ethstaker",
    "https://nitter.net/VitalikButerin",
    "https://nitter.net/Cardano",
    "https://teddit.net/r/cardano/",
    "https://nitter.net/r/CardanoMarkets/",
    "https://teddit.net/r/CardanoTrading/",
    "https://nitter.net/IOHK_Charles",
    "https://nitter.net/CardanoStiftung",
    "https://nitter.net/bigpeyYT",
    "https://nitter.net/Quantumplation",
    "https://nitter.net/atrium_lab",
    "https://nitter.net/SundaeSwap",
    "https://nitter.net/jpgstoreNFT",
    "https://nitter.net/MinswapDEX",
    "https://nitter.net/CNFT_IO",
    "https://nitter.net/wingriderscom",
    "https://nitter.net/spacebudzNFT",
    "https://nitter.net/MELD_labs",
    "https://nitter.net/pxlzNFT",
    "https://nitter.net/Tokhun_io",
    "https://nitter.net/MuesliSwapTeam",
    "https://nitter.net/Pavia_io",
    "https://nitter.net/ergoplatformorg",
    "https://teddit.net/r/ergonauts/",
    "https://teddit.net/r/erg_miners/",
    "https://teddit.net/r/ErgoDevs/",
    "https://nitter.net/chepurnoy",
    "https://nitter.net/ergofoundation",
    "https://nitter.net/danieltetsuyama",
    "https://nitter.net/AGlasgow12",
    "https://nitter.net/CuriaCrypto",
    "https://nitter.net/sigmanaut",
    "https://nitter.net/sigmaverse_ergo",
    "https://nitter.net/Spectrumlabs_",
    "https://nitter.net/coinbureau",
    "https://nitter.net/Bitboy_Crypto",
    "https://nitter.net/intocryptoverse",
    "https://nitter.net/saylor",
    "https://invidious.projectsegfau.lt/api/v1/channels/UCqK_GSMbpiV8spgD3ZGloSw"
]

for u in URLS:
    p = "dls/"+hashlib.sha256(u.encode('utf-8')).hexdigest()+"/"+timeFolder()+"a.html"
    os.system('curl "'+u+'" --create-dirs -o '+p)