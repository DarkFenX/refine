from __future__ import annotations

import json
from typing import TYPE_CHECKING

import requests

if TYPE_CHECKING:
    from tests.fw.api.client import ApiClientBase
    from tests.fw.response import Response


class Request(requests.PreparedRequest):

    def __init__(self, *, client: ApiClientBase, **kwargs):  # pylint: disable=W0231
        prepared_request = requests.Request(**kwargs).prepare()
        self.__dict__.update(prepared_request.__dict__)
        self.__client = client
        self.__body_bytes = None
        self.body = prepared_request.body

    @property
    def body(self) -> str:
        if self.__body_bytes is None:
            return ''
        return self.__body_bytes.decode('utf-8')

    @body.setter
    def body(self, body: str | bytes) -> None:
        if body is None:
            self.__body_bytes = None
            self.headers['content-length'] = str(0)
            return
        if not isinstance(body, bytes):
            body = body.encode('utf-8')
        self.headers['content-length'] = str(len(body))
        self.__body_bytes = body

    @property
    def json(self):
        return json.loads(self.body)

    @json.setter
    def json(self, data):
        self.body = json.dumps(data)

    def send(self) -> Response:
        return self.__client.send_prepared(req=self)
