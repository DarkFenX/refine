from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler


class TestDataServer(ThreadingHTTPServer):

    def __init__(self, port):
        super().__init__(('', port), TestRequestHandler)
        self.serve_forever()


class TestRequestHandler(BaseHTTPRequestHandler):
    pass

