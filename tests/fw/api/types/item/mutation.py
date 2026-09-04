import dataclasses
import typing

from fw.api.types.helpers import attr_http_to_fw
from fw.util import AttrDict, AttrHookDef


class ItemMutation(AttrDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__(data=data, hooks={
            'attrs': AttrHookDef(func=lambda d: {
                attr_http_to_fw(attr_id=k): AttrMutation(data=v) for k, v in d.items()}),
            'rolls': AttrHookDef(func=lambda d: {attr_http_to_fw(attr_id=k): v for k, v in d.items()})})


@dataclasses.dataclass
class AttrMutation:

    roll: float | None
    absolute: float

    def __init__(self, *, data: list | tuple) -> None:
        self.roll, self.absolute = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.roll, self.absolute] == other
