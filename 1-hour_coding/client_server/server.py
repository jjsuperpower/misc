import socket
from datetime import datetime
import sys

BUFF_SIZE = 2048
MAX_CHARS = 1000
IP_ADDRESS = sys.argv[1]
PORT = 5000

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1) # reuse port if already run
s.bind((IP_ADDRESS, PORT))
s.listen(1)
print('Started Server')

connect, address = s.accept()
connect.setblocking(True)

STATUS_MSG = f"Connection from {address} has been established"

print(STATUS_MSG)
for i in range(len(STATUS_MSG) - 1):
    print("-", end="")
print("-")

while(1):
    try:
        rx_msg = connect.recv(BUFF_SIZE)
        if not rx_msg:
            raise socket.error
        print(f"From Client at {datetime.now().strftime('%Y-%m-%d_%H:%M:%S')}> {rx_msg.decode()}")

        print("> ", end='')
        tx_msg = str(input())
        tx_msg = tx_msg[0:MAX_CHARS]
        tx_msg = tx_msg + '\n'
        connect.send(tx_msg.encode())

    except (socket.error):
        print("Connection Error, waiting for client")
        s.listen(1)
        connect, address = s.accept()
        connect.setblocking(True)
        print(STATUS_MSG)
        for i in range(len(STATUS_MSG) - 1):
            print("-", end="")
        print("-")
