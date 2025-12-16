
def cast_to_int(*, val: str | None) -> int | str | None:
    if val is None:
        return None
    try:
        return int(val)
    except ValueError:
        return val
