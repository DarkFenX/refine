from collections import UserList


# Entity list with a few extra access methods
class NttList(UserList):

    def first(self):  # noqa: ANN201
        assert len(self) >= 1
        return self[0]

    def one(self):  # noqa: ANN201
        assert len(self) == 1
        return self[0]

    def map(self, func):  # noqa: ANN001, ANN201
        return [func(i) for i in self]
