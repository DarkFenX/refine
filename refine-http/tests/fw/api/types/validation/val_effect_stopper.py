from collections import UserDict

from fw.api.types.helpers import effect_http_to_fw


class ValEffectStopperFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: sorted(effect_http_to_fw(e) for e in v) for k, v in data.items()})
