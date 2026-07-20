
def cast_to_int(*, val: str | None) -> int | str | None:
    if val is None:
        return None
    try:
        return int(val)
    except ValueError:
        return val


def cast_prefixed_to_int(*, val: str | None, prefix: str) -> int | str | None:
    if val is None:
        return None
    if not val.startswith(prefix):
        return val
    val = val[len(prefix):]
    try:
        return int(val)
    except ValueError:
        return val


def cast_to_prefixed_str(*, val: str | int | None, prefix: str) -> str | None:
    if val is None:
        return None
    if isinstance(val, str):
        return val
    return f'{prefix}{val}'
