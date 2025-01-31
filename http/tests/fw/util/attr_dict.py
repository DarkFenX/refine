from __future__ import annotations

import typing

# Rename it just to avoid confusion, as it's used to mean no value in this module
from .singletons import Default as NoValue

if typing.TYPE_CHECKING:
    from collections.abc import Callable


def convert(*, data: dict | tuple | list) -> AttrDict | tuple | list:
    if isinstance(data, dict):
        return AttrDict(data=data)
    if isinstance(data, tuple):
        return tuple(convert(data=i) for i in data)
    if isinstance(data, list):
        return [convert(data=i) for i in data]
    return data


class AttrHookDef:

    def __init__(self, *, func: Callable, default: Callable = lambda: NoValue) -> None:
        self.func: Callable = func
        self.default: Callable = default

    @property
    def provides_default(self) -> bool:
        # This is confusing, but Default means lack of default value here
        return self.default() is not NoValue


class AttrDict:

    def __init__(self, *, data: dict, hooks: dict[str, AttrHookDef] | None = None) -> None:
        self._data: dict = data
        self.__hooks: dict[str, AttrHookDef] = hooks or {}

    def __getitem__(self, index: int) -> typing.Any:
        return convert(data=self._data[index])

    def __getattr__(self, key: str) -> typing.Any:
        hook = self.__hooks.get(key)
        val = self._data.get(key, NoValue)
        # No value on data or default on hook raises an error
        if val is NoValue and hook is not None and hook.provides_default:
            val = hook.default()
        if val is NoValue:
            hook_keys = {k for k, v in self.__hooks.items() if v.provides_default}
            data_keys = set(self._data.keys())
            keys = sorted(hook_keys.union(data_keys))
            msg = f"no key '{key}' in keys {keys}"
            raise AttributeError(msg)
        if hook is not None:
            return hook.func(val)
        return convert(data=val)

    def __len__(self) -> int:
        return len(self._data)

    def __repr__(self) -> str:
        return f'{type(self).__name__}({self._data!r})'
