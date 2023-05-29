import contextlib
import socket


def next_free_port(start_port):
    for port in range(start_port, 65536):
        if check_port_free(port=port):
            return port
    raise RuntimeError('unable to find free port')


def check_port_free(port):
    with contextlib.closing(socket.socket(socket.AF_INET, socket.SOCK_STREAM)) as s:
        try:
            s.bind(('0.0.0.0', port))
        except OSError:
            return False
        return True
