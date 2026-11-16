from collections import UserDict

from fw.consts import ApiValItemStatus


class ValNotLoadedItemFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ApiValItemStatus(v) for k, v in data.items()})
