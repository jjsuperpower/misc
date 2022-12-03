# turn on led on esp32-s3
from machine import Pin
led = Pin(2, Pin.OUT)
led.value(0)