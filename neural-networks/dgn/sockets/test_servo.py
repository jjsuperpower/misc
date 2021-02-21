import socket
import pickle
from time import sleep

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

s.connect(('raspberrypi.sanderson.com', 5555))
print(s.recv(1024).decode())



# s.send(bytes("reset", "utf-8"))
# print(pickle.loads(s.recv(1024)))

# sleep(2)

# s.send(bytes("step", "utf-8"))
# s.send(pickle.dumps(0))
# print(pickle.loads(s.recv(1024)))

# sleep(2)

# s.send(bytes("step", "utf-8"))
# s.send(pickle.dumps(1))
# print(pickle.loads(s.recv(1024)))

# # sleep(2)

# s.send(bytes("step", "utf-8"))
# s.send(pickle.dumps(.5))
# print(pickle.loads(s.recv(1024)))

# s.send(bytes("step", "utf-8"))
# s.send(pickle.dumps(.55))
# print(pickle.loads(s.recv(1024)))

# s.send(bytes("step", "utf-8"))
# s.send(pickle.dumps(.5))
# print(pickle.loads(s.recv(1024)))

s.send(bytes("stop", "utf-8"))
print(s.recv(1024).decode())


# sleep(2)

# s.send(bytes("reset", "utf-8"))
# print(pickle.loads(s.recv(1024)))
