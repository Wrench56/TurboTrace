import constants
import time

def initial_com(conn) -> int:
    print('[*] Starting initial communication...')
    _verification(conn)
    timestamp = _initial_timestamp(conn)
    print('[*] Initial communication done!')
    return timestamp

def _verification(conn) -> None:
    message = conn.recv(3)
    while message != constants.ERR_CODE:
        if message == constants.VER_CODE:
            conn.send(constants.VERIF_CODE)
        if message == constants.ACK_CODE:
            print('   [+] Verification done!')
            return
        message = conn.recv(3)
    print('   [!] Error during verification!')
    exit(1)

def _initial_timestamp(conn) -> int:
    message = conn.recv(3)
    while message != constants.ERR_CODE:
        if message == constants.ITS_CODE:
            timestamp = current_ms_time()
            conn.send(timestamp.to_bytes(16, byteorder='big'))
        if message == constants.ACK_CODE:
            print('   [+] Initial timestamp sent!')
            return timestamp
        message = conn.recv(3)
    print('   [!] Error during initial timestamp send!')
    exit(1)

def current_ms_time():
    return round(time.time() * 1000)