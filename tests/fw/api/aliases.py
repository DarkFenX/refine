import typing
from collections.abc import Callable

if typing.TYPE_CHECKING:
    from fw.request import Request


type DpsProfileAlias = tuple[float, float, float, float] | tuple[float, float, float, float, tuple[float, float] | None]
type MutaAdd = int | tuple[int, dict[int | str, float | str]]
type MutaChange = dict[int | str, float | str | None]
type InfoMode[T] = T | tuple[T, list[tuple[T, list[str]]]]
type StatsOptions[T] = T | tuple[T, list[tuple[T, list[str]]]]

type ReqHook = Callable[[Request], None]
