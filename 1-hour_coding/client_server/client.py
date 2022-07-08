import socket
from datetime import date, datetime
from time import sleep
import sys

BUFF_SIZE = 2048
MAX_CHARS = 1000
PORT = 5000
IP_ADDRESS = sys.argv[1]
STATUS_MSG = f"Connection from {IP_ADDRESS} has been established"

connect = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
connect.setblocking(True)
connect.connect((IP_ADDRESS, PORT))

print('Started Client')
print(STATUS_MSG)
for i in range(len(STATUS_MSG) - 1):
    print("-", end="")
print("-")

while(1):
    try:
        print("> ", end='')
        tx_msg = str(input())
        tx_msg = tx_msg[0:MAX_CHARS]
        tx_msg = tx_msg + '\n'
        connect.send(tx_msg.encode())

        rx_msg = connect.recv(BUFF_SIZE)
        if not rx_msg:
            raise socket.error
        print(f"From Server at {datetime.now().strftime('%Y-%m-%d_%H:%M:%S')}> {rx_msg.decode()}")

    except (socket.error):
        print("Connection Error: Reconnecting to server")
        connect.close()
        while(connect.fileno() == -1):
            sleep(2)
            try:
                connect = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
                connect.setblocking(True)
                connect.connect((IP_ADDRESS, PORT))
            except:
                connect.close()

        print(STATUS_MSG)
        for i in range(len(STATUS_MSG) - 1):
            print("-", end="")
        print("-")
