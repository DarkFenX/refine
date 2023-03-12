from util import Default


class TestDataConsistencyError(Exception):
    pass


def conditional_insert(container, key, value):
    if value is not Default:
        container[key] = value
