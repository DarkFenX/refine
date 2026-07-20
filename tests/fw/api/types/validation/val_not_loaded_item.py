from collections import UserList


class ValNotLoadedItemFail(UserList):

    def __init__(self, *, data: list | tuple) -> None:
        super().__init__(sorted(data))
