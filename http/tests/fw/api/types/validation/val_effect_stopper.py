from collections import UserDict


class ValEffectStopperFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: sorted(v) for k, v in data.items()})
