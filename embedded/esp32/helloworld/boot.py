# boot.py -- run on boot-up
import machine
from utils import logging
import time

logging.debug(f"Current frequency: {machine.freq()}")
machine.freq(int(240e6))
logging.debug("New frequency: " + str(machine.freq()))

logging.info('#############################')
logging.info('###       Booting Up      ###')
logging.info('#############################')