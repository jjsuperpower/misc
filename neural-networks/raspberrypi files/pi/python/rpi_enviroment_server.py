from mpu6050 import mpu6050
from time import sleep, time
import random
import pickle
import pigpio
import socket

print('Imported Libraries')



class Enviroment:
    def __init__(self):

        self.ACCEL_SAMPLE_SIZE = 32
        self.GYRO_SAMPLE_SIZE = 32
        self.SENSOR_TOLERANCE = 1
        self.SENSOR_TARGET_POS = 0

        self.SERVO_0_PIN = 23
        self.SERVO_1_PIN = 24
        self.SERVO_MIN = 800
        self.SERVO_MAX = 2500
        self.SERVO_TIME_DELAY_MIN = 1
        self.SERVO_TIME_DELAY_MAX = 1.5

        self.ACTION_REWARD_FACTOR = 100
        self.ACTION_REWARD_MAX = 150
        self.TIME_PEN_FACTOR = 0

        self.servo = pigpio.pi()
        self.servo.set_servo_pulsewidth(self.SERVO_0_PIN, 1650)
        self.servo.set_servo_pulsewidth(self.SERVO_1_PIN, 1650)

        self.sensor = mpu6050(0x68)

        self.time_start = 0
        self.time_stop = 0

    def _servo_set_pos(self, servo_pin, servo_new_pos):
        if (0 <= servo_new_pos <= 1):
            servo_current_pos = self.servo.get_servo_pulsewidth(servo_pin)

            servo_value = (self.SERVO_MAX - self.SERVO_MIN) * servo_new_pos + self.SERVO_MIN
            self.servo.set_servo_pulsewidth(servo_pin, servo_value)
            
            delay = abs(servo_new_pos - (servo_current_pos - self.SERVO_MIN) / (self.SERVO_MAX - self.SERVO_MIN))  * self.SERVO_TIME_DELAY_MAX
            if (delay < self.SERVO_TIME_DELAY_MIN):
                delay = self.SERVO_TIME_DELAY_MIN
            if (delay > self.SERVO_TIME_DELAY_MAX):
                delay = self.SERVO_TIME_DELAY_MAX
            # print(f"Delay is {delay}")
            sleep(delay)

    def reset(self):
        servo_new_pos = random.uniform(.1, .9)
        self._servo_set_pos(self.SERVO_0_PIN, servo_new_pos)
        self.time_start = time()

        new_state = [self._get_accel_avg(), self.servo.get_servo_pulsewidth(self.SERVO_1_PIN), self.SENSOR_TARGET_POS]
        return new_state

    def stop(self):
        self._servo_set_pos(self.SERVO_0_PIN, 1650)
        self._servo_set_pos(self.SERVO_1_PIN, 1650)

    def _get_accel_avg(self):
        accel_list = []
        for i in range(self.ACCEL_SAMPLE_SIZE):
            accel_list.append(self.sensor.get_accel_data()['y'])
        return sum(accel_list) / len(accel_list)
        
    def _get_gyro_avg(self):
        gyro_list = []
        for i in range(self.GYRO_SAMPLE_SIZE):
            gyro_list.append(self.sensor.get_gyro_data()['y'])
        return sum(gyro_list) / len(gyro_list)

    def step(self, action):

        self._servo_set_pos(self.SERVO_1_PIN, action)
        self.servo_1_pos = action

        reward = (self.SENSOR_TOLERANCE / abs(self.SENSOR_TARGET_POS - self._get_accel_avg())) * self.ACTION_REWARD_FACTOR
        penalty = 0

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

        new_state = [self._get_accel_avg(), self.servo_1_pos, self.SENSOR_TARGET_POS]
        return new_state, reward - penalty, done


env = Enviroment()

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.bind(('raspberrypi.sanderson.com', 5555))
s.listen(1)
print('Started server')

while True:
    clientsocket, address = s.accept()
    print(f"Connection from {address} has been established")
    clientsocket.send(bytes(f"Connected to server at {address}", "utf-8"))

    msg_recieved = clientsocket.recv(1024).decode()
    while msg_recieved:
        
        if (msg_recieved == 'reset'):
            msg_send = pickle.dumps(env.reset())
            clientsocket.send(msg_send)

        elif (msg_recieved == 'stop'):
            env.stop()
            clientsocket.send(bytes(f"Done", "utf-8"))

        elif (msg_recieved == 'step'):
            data = clientsocket.recv(1024)
            action = pickle.loads(data)
            msg_send = pickle.dumps(env.step(action))
            clientsocket.send(msg_send)

        else:
            print("There was an error in comunication")
            clientsocket.send(bytes("error", "utf-8"))
        
        msg_recieved = clientsocket.recv(1024).decode()