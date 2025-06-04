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


def compare(*, left: typing.Any, right: typing.Any) -> bool:
    if type(left) is not type(right):
        return False
    if isinstance(left, AttrDict) and isinstance(right, AttrDict):
        return compare_attr_dict(left=left, right=right)
    if isinstance(left, list | tuple) and isinstance(right, list | tuple):
        return compare_sequence(left=left, right=right)
    return left == right


def compare_attr_dict(*, left: AttrDict, right: AttrDict) -> bool:
    fields_left = set(left.get_raw().keys())
    fields_right = set(right.get_raw().keys())
    if fields_left != fields_right:
        return False
    for field_name in fields_left:
        sub_left = getattr(left, field_name)
        sub_right = getattr(right, field_name)
        if not compare(left=sub_left, right=sub_right):
            return False
    return True


def compare_sequence(*, left: list | tuple, right: list | tuple) -> bool:
    if len(left) != len(right):
        return False
    for i, sub_left in enumerate(left):
        sub_right = right[i]
        if not compare(left=sub_left, right=sub_right):
            return False
    return True


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

    def get_raw(self) -> dict:
        return self._data

    def compare(self, *, other: AttrDict) -> bool:
        return compare(left=self, right=other)

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
