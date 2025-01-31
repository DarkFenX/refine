from __future__ import annotations

import typing

from .singletons import Absent, Default

if typing.TYPE_CHECKING:
    from collections.abc import Callable


def conditional_insert(
        *,
        container: dict,
        key: str,
        value: typing.Any,
        cast_to: Callable | None = None,
) -> None:
    if value is Default:
        msg = 'value should not be Default'
        raise ValueError(msg)
    if value is Absent:
        return
    if value is not None and cast_to is not None:
        value = cast_to(value)
    container[key] = value
