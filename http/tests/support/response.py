from __future__ import annotations

from typing import TYPE_CHECKING

import requests

if TYPE_CHECKING:
    from typing import Union


class Response(requests.Response):

    def __init__(self, *, response: requests.Response):  # pylint: disable=W0231
        self.__dict__.update(response.__dict__)

    def check(
            self, *,
            status_code: Union[int, None] = None,
            text_predicate: Union[str, None] = None,
            json_predicate: Union[dict, None] = None,
    ) -> None:
        if status_code is not None:
            assert self.status_code == status_code
        if text_predicate is not None:
            assert self.text == text_predicate
        if json_predicate is not None:
            assert self.json() == json_predicate
