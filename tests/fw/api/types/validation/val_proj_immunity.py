import dataclasses
import typing
from collections import UserDict

from fw.consts import ApiValItemStatus


class ValProjImmunityFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValProjImmunityItemInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValProjImmunityItemInfo:

    status: ApiValItemStatus
    projectee_item_ids: list[str]

    def __init__(self, *, data: list | tuple) -> None:
        status, projectee_item_ids = data
        self.status = ApiValItemStatus(status)
        self.projectee_item_ids = sorted(projectee_item_ids)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.projectee_item_ids] == other
