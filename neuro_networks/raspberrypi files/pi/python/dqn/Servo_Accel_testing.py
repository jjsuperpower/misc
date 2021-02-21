from mpu6050 import mpu6050
import RPi.GPIO as GPIO

import math
from time import sleep, time
import numpy as np

SERVO_DUTY_MIN = 3
SERVO_DUTY_MAX = 11

SERVO_PIN = 18
SERVO_FREQ = 50

sensor = mpu6050(0x68)

GPIO.setwarnings(False)
GPIO.setmode(GPIO.BCM)
GPIO.setup(SERVO_PIN, GPIO.OUT)

servo = GPIO.PWM(SERVO_PIN, SERVO_FREQ)

servo_duty_cycle = 0
servo.start(servo_duty_cycle)
raw_accel = []
raw_gyro = []

while(servo_duty_cycle != -1):
    sleep(1)

    for i in range(20):
        raw_accel.append(sensor.get_accel_data()['y'])
    min_accel = min(raw_accel)
    avg_accel = sum(raw_accel) / len(raw_accel)
    max_accel = max(raw_accel)
    raw_accel = []

    for i in range(20):
        raw_gyro.append(sensor.get_gyro_data()['y'])
    min_gyro = min(raw_gyro)
    avg_gyro = sum(raw_gyro) / len(raw_gyro)
    max_gyro = max(raw_gyro)
    raw_gyro = []

    print("y-axis accel min: " + str(min_accel))
    print("y-axis accel avg: " + str(avg_accel))
    print("y-axis accel max: " + str(max_accel))
    print('')
    print("y-axis gyro min: " + str(min_gyro))
    print("y-axis gyro avg: " + str(avg_gyro))
    print("y-axis gyro max: " + str(max_gyro))
    print('')
    

    servo_duty_cycle = float(input('Duty Cycle: '))
    if (servo_duty_cycle != -1):
        servo.ChangeDutyCycle(servo_duty_cycle)

servo.stop()



# servo.start(0)
# for i in range(0, 15, 1):
#     print(i)
#     sleep(1)
#     servo.ChangeDutyCycle(i)
# servo.stop()



# x_sens_array = np.zeros((1000,))
# for i in range(0, 999):
#     x_sens = sensor.get_accel_data()['x']
#     x_sens_array[i] = x_sens
#     if not i % 100:
#         print(x_sens)
# np.save('x_sens_arra', x_sens_array)