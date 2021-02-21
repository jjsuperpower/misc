from mpu6050 import mpu6050
import RPi.GPIO as GPIO

# import keras.backend.tensorflow_backend as backend
# from keras.models import Sequential
# from keras.layers import Dense, Dropout, Conv2D, MaxPooling2D, Activation, Flatten
# from keras.optimizers import Adam
# from keras.callbacks import TensorBoard

from tensorforce.agents import Agent
from tensorforce.environments import Environment
from tensorforce.execution import Runner

import math
from time import sleep, time
import random
import numpy as np
import os

AGENT = 'ppo'

print('Starting tensorboard')
stream = os.popen('tensorboard --logdir summary/')
print(stream.read())

class Enviroment:
    def __init__(self):

        self.ACCEL_SAMPLE_SIZE = 32
        self.GYRO_SAMPLE_SIZE = 32
        self.SENSOR_TOLERANCE = 1
        self.SENSOR_TARGET_POS = 0

        self.SERVO_0_PIN = 22
        self.SERVO_1_PIN = FIX
        self.SERVO_FREQ = 50
        self.SERVO_DUTY_MIN = 3
        self.SERVO_DUTY_MAX = 11
        self.SERVO_TIME_DELAY_MIN = .2
        self.SERVO_TIME_DELAY_MAX = .7

        self.ACTION_REWARD_FACTOR = 100
        self.ACTION_REWARD_MAX = 150
        self.TIME_PEN_FACTOR = 0

        GPIO.setwarnings(False)
        GPIO.setmode(GPIO.BOARD)
        GPIO.setup(self.SERVO_0_PIN, GPIO.OUT)
        GPIO.setup(self.SERVO_1_PIN, GPIO.OUT)

        self.servo_0 = GPIO.PWM(self.SERVO_0_PIN, self.SERVO_FREQ)
        self.servo_1 = GPIO.PWM(self.SERVO_1_PIN, self.SERVO_FREQ)
        self.sensor = mpu6050(0x68)

        self.servo_0_pos = 0
        self.servo_1_pos = 0
        self.time_start = 0
        self.time_stop = 0

    def servo_set_pos(self, servo, servo_current_pos, servo_new_pos):
        if(0 <= servo_new_pos <= 1):
            servo.ChangeDutyCycle((self.SERVO_DUTY_MAX - self.SERVO_DUTY_MIN) * servo_new_pos + self.SERVO_DUTY_MIN)
            
        delay = abs(servo_new_pos - servo_current_pos) * self.SERVO_TIME_DELAY_MAX
        if (delay < self.SERVO_TIME_DELAY_MIN):
            delay = self.SERVO_TIME_DELAY_MIN
        if (delay > self.SERVO_TIME_DELAY_MAX):
            delay = self.SERVO_TIME_DELAY_MAX
        sleep(delay)

    def reset(self):
        servo_new_pos = np.random.random()
        self.servo_set_pos(self.servo_0_pos, self.servo_0_pos, servo_new_pos)
        self.servo_0_pos = servo_new_pos
        self.time_start = time()

        new_state = [self.get_accel_avg(), self.servo_1_pos, self.SENSOR_TARGET_POS]
        return new_state

    def stop(self):
        self.servo_0_pos.stop()
        self.servo_0_pos.stop()

    def get_accel_avg(self):
        accel_list = []
        for i in range(self.ACCEL_SAMPLE_SIZE):
            accel_list.append(self.sensor.get_accel_data()['y'])
        return sum(accel_list) / len(accel_list)
        
    def get_gyro_avg(self):
        gyro_list = []
        for i in range(self.GYRO_SAMPLE_SIZE):
            gyro_list.append(self.sensor.get_gyro_data()['y'])
        return sum(gyro_list) / len(gyro_list)

    def step(self, action):

        self.servo_set_pos(self.servo_1, self.servo_1_pos, action)
        self.servo_1_pos = action

        reward = (self.SENSOR_TOLERANCE / abs(self.SENSOR_TARGET_POS - self.get_accel_avg())) * self.ACTION_REWARD_FACTOR

        if (reward > self.ACTION_REWARD_MAX):
            reward = self.ACTION_REWARD_MAX

        if (reward >= self.ACTION_REWARD_FACTOR):

            self.time_stop = time()
            time_dif = (self.time_stop - self.time_start) * self.TIME_PEN_FACTOR

            reward += self.ACTION_REWARD_FACTOR
            penalty = time_dif * self.TIME_PEN_FACTOR
            done = True

        else:
            done = False

        # print('Reward = ' + str(int(reward)))

        new_state = [self.get_accel_avg(), self.servo_1_pos, self.SENSOR_TARGET_POS]
        return new_state, reward - penalty, done


class Enviroment_API:
    def __init__(self):
        self.env = Enviroment()
    def states(self):
        return dict(type='float', shape=(3,))
    def actions(self):
        return dict(type='float', shape=(1,), min_value=0, max_value=1)
    def close(self):
        self.env.stop()
    def reset(self):
        return dict(state=self.env.reset())
    def execute(self, action):
        new_state, reward, done = self.env.step(action['action'])
        return dict(state=new_state), done, reward


env = Enviroment_API()

agent = Agent.create(
                    agent=AGENT,
                    evironment=env,
                    memory=100,
                    summarizer=dict(path='data/summary'),
                    saver=dict(path='data/agent')
                    )

runner = Runner(
                agent=AGENT,
                environment=env
)

runner.run(num_episodes=100)
runner.run(num_episodes=20, evaluation=True)
runner.close()