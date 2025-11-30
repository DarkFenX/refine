import dataclasses
import typing


@dataclasses.dataclass
class ValOverloadSkillFail:

    td_lvl: int | None
    module_reqs: dict[str, int]

    def __init__(self, *, data: list | tuple) -> None:
        self.td_lvl, self.module_reqs = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.td_lvl, self.module_reqs] == other
