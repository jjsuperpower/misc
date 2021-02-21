import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

s.connect(('raspberrypi.sanderson.com', 5555))

msg = s.recv(1024)
print(msg.decode("utf-8"))

s.send(bytes("1", "utf-8"))
print(s.recv(1024).decode())

s.send(bytes("2", "utf-8"))
print(s.recv(1024).decode())

s.send(bytes("3", "utf-8"))
print(s.recv(1024).decode())

s.send(bytes("4", "utf-8"))
print(s.recv(1024).decode())

s.send(bytes("5", "utf-8"))
print(s.recv(1024).decode())