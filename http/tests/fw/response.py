import requests


class Response(requests.Response):

    def __init__(self, *, response: requests.Response) -> None:
        self.__dict__.update(response.__dict__)

    def check(
            self, *,
            status_code: int | None = None,
            text_predicate: str | None = None,
            json_predicate: dict | None = None,
    ) -> None:
        if status_code is not None:
            assert self.status_code == status_code
        if text_predicate is not None:
            assert self.text == text_predicate
        if json_predicate is not None:
            assert self.json() == json_predicate
