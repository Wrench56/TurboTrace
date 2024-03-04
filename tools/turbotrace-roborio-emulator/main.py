import socket

import payload
import utils

from art import E_B, I_B, Q_B

def main():
    print(f'{I_B} Starting TurboTrace RoboRIO Emulator...')
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(('0.0.0.0', 40025))
    print(f'{I_B} Waiting for connection...')
    sock.listen()
    conn, addr = sock.accept()
    with conn:
        print(f'{I_B} {addr[0]}:{addr[1]} connected!')
        timestamp = utils.initial_com(conn)
        print(f'{"="*15}[LOOP]{"="*15}\n')
        while True:
            _payload = payload.Payload()
            _payload.set_level(input(f'{Q_B} Enter level:\n * [ LVL ] >>> '))
            _payload.set_timestamp(timestamp)
            print(f'\n{_payload}')
            print(f'{"="*15}[SENT]{"="*15}\n')

            conn.send(_payload.send())
        conn.close()

if __name__ == '__main__':
    while True:
        try:
            main()
        except ConnectionResetError:
            print(f'{E_B} Pipe broken, rolling back...')
        except KeyboardInterrupt:
            break