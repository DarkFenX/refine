import re
import typing

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
            subset_check(actual=self.json(), expected=json_predicate)


def subset_check(*, actual: typing.Any, expected: typing.Any) -> None:
    if isinstance(actual, dict) and isinstance(expected, dict):
        for name in expected:
            subset_check(actual=actual.get(name), expected=expected.get(name))
        return
    if isinstance(actual, list | tuple) and isinstance(expected, list | tuple):
        assert len(actual) == len(expected)
        for i in range(len(expected)):
            subset_check(actual=actual[i], expected=expected[i])
        return
    # Regex matching based on "re:" string prefix
    if isinstance(expected, str) and expected[:3] == 're:':
        pattern = expected[3:]
        assert re.match(pattern, actual) is not None
        return
    assert actual == expected
