from collections import UserList


# Entity list with a few extra access methods
class NttList(UserList):

    # ruff:ignore[missing-return-type-undocumented-public-function]
    def first(self):
        assert len(self) >= 1
        return self[0]

    # ruff:ignore[missing-return-type-undocumented-public-function]
    def one(self):
        assert len(self) == 1
        return self[0]

    # ruff:ignore[missing-type-function-argument, missing-return-type-undocumented-public-function]
    def map(self, func):
        return [func(i) for i in self]
