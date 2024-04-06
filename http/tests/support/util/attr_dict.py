

def convert(data, hooks):
    if isinstance(data, dict):
        return AttrDict(data=data, hooks=hooks)
    if isinstance(data, tuple):
        return tuple(convert(data=i, hooks=hooks) for i in data)
    if isinstance(data, list):
        return list(convert(data=i, hooks=hooks) for i in data)
    return data


class AttrDict:

    def __init__(self, data, hooks=None):
        self._data = data
        self._hooks = hooks or {}

    def __getitem__(self, index: int):
        return convert(data=self._data[index], hooks=self._hooks)

    def __getattr__(self, key: str):
        default = object()
        val = self._data.get(key, default)
        if val is default:
            keys = sorted(self._data.keys())
            raise AttributeError(f"no key '{key}' in keys {keys}")
        hook = self._hooks.get(key)
        if hook is not None:
            return hook(val)
        return convert(data=val, hooks=self._hooks)

    def __len__(self):
        return len(self._data)

    def __repr__(self) -> str:
        return f'{type(self).__name__}({repr(self._data)})'
