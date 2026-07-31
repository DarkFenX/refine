import json
import typing

import requests

if typing.TYPE_CHECKING:
    from fw.api.client.base import ApiClientBase
    from fw.response import Response


class Request:

    def __init__(self, *, client: ApiClientBase, **kwargs) -> None:
        self.__client = client
        self.__prepared = requests.Request(**kwargs).prepare()

    @property
    def prepared(self) -> requests.PreparedRequest:
        return self.__prepared

    # Returns body size in _bytes_
    def get_body_size(self) -> int:
        if self.__prepared.body is None:
            return 0
        if not isinstance(self.__prepared.body, bytes):
            return len(self.__prepared.body.encode('utf-8'))
        return len(self.__prepared.body)

    # Returns body as string
    def get_body(self) -> str:
        if self.__prepared.body is None:
            return ''
        if isinstance(self.__prepared.body, bytes):
            return self.__prepared.body.decode('utf-8')
        return self.__prepared.body

    def set_body(self, *, body: str | bytes | None, chunk: int | None = None) -> None:
        # No body
        if body is None:
            if chunk:
                self.__prepared.headers.pop('Content-Length', None)
            else:
                self.__prepared.headers['Content-Length'] = str(0)
            self.__prepared.body = None
            return
        if not isinstance(body, bytes):
            body = body.encode('utf-8')
        # With body, chunked
        if chunk:
            self.__prepared.headers.pop('Content-Length', None)
            self.__prepared.body = iter([body[i:i + chunk] for i in range(0, len(body), chunk)])
        # With body, non-chunked
        else:
            self.__prepared.headers['Content-Length'] = str(len(body))
            self.__prepared.body = body

    def get_json(self) -> typing.Any:
        body = self.get_body()
        return json.loads(body)

    def set_json(self, *, data: typing.Any, chunk: int | None = None) -> None:
        body = json.dumps(data)
        self.set_body(body=body, chunk=chunk)

    def send(self) -> Response:
        return self.__client.send_prepared(req=self)
