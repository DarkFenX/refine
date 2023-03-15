from ....util import Absent


class TestDataConsistencyError(Exception):
    pass


def conditional_insert(container, key, value):
    if value is not Absent:
        container[key] = value
