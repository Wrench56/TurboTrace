import socket

import payload
import utils

def main():
    print('[+] Starting TurboTrace RoboRIO Emulator...')
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind(('0.0.0.0', 40025))
    print('[+] Waiting for connection...')
    sock.listen()
    conn, addr = sock.accept()
    with conn:
        print(f'[+] {addr[0]}:{addr[1]} connected!')
        timestamp = utils.initial_com(conn)
        print('===========[LOAD]===========\n')
        while True:
            _payload = payload.Payload()
            _payload.set_level(input('[?] Enter level:\n * [ LVL ] >>> '))
            _payload.set_timestamp(timestamp)
            print(f'\n{_payload}')
            print('===========[SENT]===========\n')

            conn.send(_payload.send())

        conn.close()

if __name__ == '__main__':
    while True:
        try:
            main()
        except ConnectionResetError:
            print('[!] Pipe broken, rolling back...')
        except KeyboardInterrupt:
            break