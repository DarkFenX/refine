from .singletons import Default as NoValue


def convert(data, hooks):
    if isinstance(data, dict):
        return AttrDict(data=data, hooks=hooks)
    if isinstance(data, tuple):
        return tuple(convert(data=i, hooks=hooks) for i in data)
    if isinstance(data, list):
        return list(convert(data=i, hooks=hooks) for i in data)
    return data


class AttrHookDef:

    def __init__(self, func, default=NoValue):
        self.func = func
        self.default = default

    @property
    def provides_default(self):
        # This is confusing, but Default means lack of default value here
        return self.default is not NoValue


class AttrDict:

    def __init__(self, data, hooks=None):
        self._data = data
        self._hooks = hooks or {}

    def __getitem__(self, index: int):
        return convert(data=self._data[index], hooks=self._hooks)

    def __getattr__(self, key: str):
        hook = self._hooks.get(key)
        val = self._data.get(key, NoValue)
        # No value on data or default on hook raises an error
        if val is NoValue and hook is not None and hook.provides_default:
            val = hook.default
        if val is NoValue:
            hook_keys = set(k for k, v in self._hooks.items() if v.provides_default)
            data_keys = set(self._data.keys())
            keys = sorted(hook_keys.union(data_keys))
            raise AttributeError(f"no key '{key}' in keys {keys}")
        if hook is not None:
            return hook.func(val)
        return convert(data=val, hooks=self._hooks)

    def __len__(self):
        return len(self._data)

    def __repr__(self) -> str:
        return f'{type(self).__name__}({repr(self._data)})'
