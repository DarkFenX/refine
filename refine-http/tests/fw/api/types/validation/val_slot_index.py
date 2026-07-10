from collections import UserDict


class ValSlotIndexFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({int(k): sorted(v) for k, v in data.items()})
