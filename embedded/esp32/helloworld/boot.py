# boot.py -- run on boot-up

print('#############################')
print('###       Booting Up      ###')
print('#############################')

import machine
from utils import logging
logging.group = 'BOOT'

logging.debug(f"Current frequency: {machine.freq()}")
machine.freq(int(240e6))
logging.debug("New frequency: " + str(machine.freq()))