import json

import requests


class Request(requests.PreparedRequest):

    def __init__(self, client, *args, **kwargs):
        prepared_request = requests.Request(*args, **kwargs).prepare()
        self.__dict__.update(prepared_request.__dict__)
        self.__client = client
        self.__body_bytes = None
        self.body = prepared_request.body

    @property
    def body(self):
        if self.__body_bytes is None:
            return ''
        return self.__body_bytes.decode('utf-8')

    @body.setter
    def body(self, body):
        if body is None:
            self.__body_bytes = None
            self.headers['content-length'] = 0
            return
        if not isinstance(body, bytes):
            body = body.encode('utf-8')
        self.headers['content-length'] = len(body)
        self.__body_bytes = body

    @property
    def json(self):
        return json.loads(self.body)

    @json.setter
    def json(self, data):
        self.body = json.dumps(data)

    def send(self):
        return self.__client.send_prepared(self)
