"""
pytest-httpserver used to serve EVE data. Now, the server from this module is used instead, not to
exhaust ports available for connections during a test run on macOS. macOS runs into issues because
it has fewer ports available, and they stay unusable for a longer time after the connection using
them is closed.
"""


import socket
import socketserver
import threading
import typing
from http.server import BaseHTTPRequestHandler


class StaticHttpServer:

    def __init__(self) -> None:
        self.__server: Server = bind_server()
        self.__thread: threading.Thread = threading.Thread(target=self.__server.serve_forever, daemon=True)
        # Address of the bound socket, so that clients connect on the first try without lookups. For
        # about 4500 tests, it speeds the test run up by ~15-20 seconds
        host, port = self.__server.server_address[:2]
        if ':' in host:
            host = f'[{host}]'
        self.__base_url: str = f'http://{host}:{port}'

    def start(self) -> None:
        self.__thread.start()

    def stop(self) -> None:
        self.__server.shutdown()
        self.__server.server_close()

    def set_route(self, *, path: str, data: str) -> None:
        self.__server.routes[path] = data.encode()

    def clear(self) -> None:
        self.__server.routes.clear()

    @property
    def base_url(self) -> str:
        return self.__base_url


class Server(socketserver.ThreadingTCPServer):

    daemon_threads = True

    def __init__(self, *, family: socket.AddressFamily, host: str) -> None:
        self.address_family = family
        self.routes: dict[str, bytes] = {}
        # 0 - any available port
        super().__init__((host, 0), Handler)


class Handler(BaseHTTPRequestHandler):

    # Use 1.1 for having keep-alive enabled by default
    protocol_version = 'HTTP/1.1'
    # Without it headers & body are sent as separate packets, waiting for ACK, which slows test runs
    # too much
    disable_nagle_algorithm = True

    def do_GET(self) -> None:
        data = self.server.routes.get(self.path)
        if data is None:
            self.send_response(404)
            self.send_header('Content-Length', '0')
            self.end_headers()
            return
        self.send_response(200)
        self.send_header('Content-Length', str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    @typing.override
    def log_message(self, *args: object) -> None:
        pass


def bind_server() -> Server:
    try:
        return Server(family=socket.AF_INET, host='127.0.0.1')
    except OSError:
        return Server(family=socket.AF_INET6, host='::1')
