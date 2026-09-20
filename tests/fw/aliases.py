import typing

if typing.TYPE_CHECKING:
    from collections.abc import Callable

    from fw.request import Request


type JsonPredicate = dict | list | None
type ReqHook = Callable[[Request], None]
