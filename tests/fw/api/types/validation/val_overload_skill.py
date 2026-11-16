import dataclasses
import typing

from fw.consts import ApiValItemStatus


@dataclasses.dataclass
class ValOverloadSkillFail:

    td_lvl: int | None
    module_reqs: dict[str, ValOverloadSkillItemInfo]

    def __init__(self, *, data: list | tuple) -> None:
        self.td_lvl, module_reqs = data
        self.module_reqs = {k: ValOverloadSkillItemInfo(data=v) for k, v in module_reqs.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.td_lvl, self.module_reqs] == other


@dataclasses.dataclass
class ValOverloadSkillItemInfo:

    status: ApiValItemStatus
    level_req: int

    def __init__(self, *, data: list | tuple) -> None:
        status, self.level_req = data
        self.status = ApiValItemStatus(status)

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.level_req] == other
