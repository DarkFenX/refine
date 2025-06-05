
import dataclasses
import typing


@dataclasses.dataclass
class ValDroneGroupFail:

    allowed_group_ids: list[int]
    drone_groups: dict[str, int]

    def __init__(self, *, data: tuple) -> None:
        self.allowed_group_ids = sorted(data[0])
        self.drone_groups = data[1]

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: tuple) -> bool:
        return (self.allowed_group_ids, self.drone_groups) == (sorted(other[0]), other[1])
