from __future__ import annotations

from tests.support.consts import ApiValType
from tests.support.util import AttrDict, AttrHookDef


class ValResult(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={'details': AttrHookDef(func=lambda d: ValResultDetails(data=d))})


class ValResultDetails(AttrDict):

    def __init__(self, *, data: dict):
        super().__init__(data=data, hooks={
            ApiValType.cpu: AttrHookDef(func=lambda d: AttrDict(data=d)),
            ApiValType.pg: AttrHookDef(func=lambda d: AttrDict(data=d))})
