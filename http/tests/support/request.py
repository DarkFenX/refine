from __future__ import annotations

import json
from typing import TYPE_CHECKING

import requests

if TYPE_CHECKING:
    from typing import Union

    from tests.support.api import ApiClient
    from tests.support.response import Response


class Request(requests.PreparedRequest):

    def __init__(self, client: ApiClient, *args, **kwargs):  # pylint: disable=W0231
        prepared_request = requests.Request(*args, **kwargs).prepare()
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
    def body(self, body: Union[str, bytes]) -> None:
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

    def send(self) -> Response:
        return self.__client.send_prepared(req=self)
