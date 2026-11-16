import dataclasses
import typing
from collections import UserDict

from fw.consts import ApiValItemStatus


class ValSrqFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValSrqItemInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValSrqItemInfo:

    status: ApiValItemStatus
    missing_skills: dict[int, ValSrqSkill]

    def __init__(self, *, data: list | tuple) -> None:
        status, missing_skills = data
        self.status = ApiValItemStatus(status)
        self.missing_skills = {
            int(skill_id): ValSrqSkill(data=skill_data)
            for skill_id, skill_data in missing_skills.items()}

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.status, self.missing_skills] == other


@dataclasses.dataclass
class ValSrqSkill:

    current_lvl: int | None
    required_lvl: int

    def __init__(self, *, data: list | tuple) -> None:
        self.current_lvl, self.required_lvl = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.current_lvl, self.required_lvl] == other
