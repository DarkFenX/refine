from .singletons import Absent, Default


def conditional_insert(container, key, value, cast_to=None):
    if value is Default:
        raise ValueError('value should not be Default')
    if value is Absent:
        return
    if value is not None and cast_to is not None:
        value = cast_to(value)
    container[key] = value
