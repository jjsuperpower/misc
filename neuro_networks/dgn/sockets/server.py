import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

s.bind((socket.gethostname(), 1234))
s.listen(5)
print('Started server')

while True:
    clientsocket, address = s.accept()
    print(f"Connection from {address} has been established")
    clientsocket.send(bytes("Hello World", "utf-8"))
    msg = clientsocket.recv(1024).decode()
    while msg:
        
        if (msg == '1'):
            clientsocket.send(bytes("1", "utf-8"))
        elif (msg == '2'):
            clientsocket.send(bytes("2", "utf-8"))
        elif (msg == '3'):
            clientsocket.send(bytes("3", "utf-8"))
        elif (msg == '4'):
            clientsocket.send(bytes("4", "utf-8"))
        else:
            clientsocket.send(bytes("Unknown value", "utf-8"))
        msg = clientsocket.recv(1024).decode()