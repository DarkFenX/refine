

def convert(data, hooks):
    if isinstance(data, (dict, list)):
        return AttrDict(data=data, hooks=hooks)
    return data


class AttrDict:

    def __init__(self, data, hooks=None):
        self._data = data
        self._hooks = hooks or {}

    def __getitem__(self, index):
        return convert(data=self._data[index], hooks=self._hooks)

    def __getattr__(self, key):
        default = object()
        val = self._data.get(key, default)
        if val is default:
            keys = sorted(self._data.keys())
            raise AttributeError(f"no key '{key}' in keys {keys}")
        hook = self._hooks.get(key)
        if hook is not None:
            return hook(val)
        return convert(data=val, hooks=self._hooks)

    def __repr__(self):
        return repr(self._data)
