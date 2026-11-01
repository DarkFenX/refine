from collections import UserList


class NttRack(UserList):

    # ruff:ignore[missing-type-function-argument, missing-return-type-undocumented-public-function]
    def map(self, func):
        return [None if i is None else func(i) for i in self]
