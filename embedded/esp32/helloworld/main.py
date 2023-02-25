# turn on led on esp32-s3
from utils import logging
logging.group = 'MAIN'

from machine import Pin, PWM

def set_led():
    led = Pin(2, Pin.OUT)
    led.value(1)


def set_pwm():
    pwm2 = PWM(Pin(2), freq=20, duty=512)
    

def connect_wifi():
    import network
    import socket
    sta_if = network.WLAN(network.STA_IF)
    sta_if.active(True)
    sta_if.connect('SAND01', 'cosette2411')
    while not sta_if.isconnected():
        pass
    logging.info('Network connection established')

def ping_google():
    from uping import ping
    out = ping('google.com', count=1)
    logging.info(out)

connect_wifi()
ping_google()