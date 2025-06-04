
import dataclasses
import typing


class ValSrqFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({
            item_id: {int(skill_id): ValSrqSkill(data=skill_data) for skill_id, skill_data in item_data.items()}
            for item_id, item_data in data.items()})


@dataclasses.dataclass
class ValSrqSkill:

    current_lvl: int | None
    required_lvl: int

    def __init__(self, *, data: tuple) -> None:
        self.current_lvl = data[0]
        self.required_lvl = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.current_lvl, self.required_lvl) == (other[0], other[1])
