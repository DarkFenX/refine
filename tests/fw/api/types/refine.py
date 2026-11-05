import typing

from fw.api.types.sol import Sol
from fw.api.types.src import Src
from fw.util import AttrDict, AttrHookDef

if typing.TYPE_CHECKING:
    from fw.api import ApiClient


class RefineInfo(AttrDict):

    def __init__(self, *, client: ApiClient, data: dict) -> None:
        super().__init__(data=data, hooks={
            'srcs': AttrHookDef(func=lambda ss: {s['alias']: Src(client=client, data=s) for s in ss}),
            'sols': AttrHookDef(func=lambda ss: {s['id']: Sol(client=client, data=s) for s in ss})})
