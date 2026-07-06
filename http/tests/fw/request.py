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

    def get_body(self) -> str:
        if self.__prepared.body is None:
            return ''
        if isinstance(self.__prepared.body, bytes):
            self.__prepared.body.decode('utf-8')
        return self.__prepared.body

    def set_body(self, body: str | bytes | None) -> None:
        if body is None:
            self.__prepared.body = None
            self.__prepared.headers['Content-Length'] = str(0)
            return
        if not isinstance(body, bytes):
            body = body.encode('utf-8')
        self.__prepared.headers['Content-Length'] = str(len(body))
        self.__prepared.body = body

    def get_json(self) -> typing.Any:
        body = self.get_body()
        return json.loads(body)

    def set_json(self, data: typing.Any) -> None:
        body = json.dumps(data)
        self.set_body(body)

    def send(self) -> Response:
        return self.__client.send_prepared(req=self)
