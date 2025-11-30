import dataclasses
import typing


class ValModuleStateFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValModuleStateInfo(data=v) for k, v in data.items()})


@dataclasses.dataclass
class ValModuleStateInfo:

    state: str
    max_state: str

    def __init__(self, *, data: list | tuple) -> None:
        self.state, self.max_state = data

    def __getitem__(self, item: int) -> typing.Any:
        field = dataclasses.fields(self)[item]
        return getattr(self, field.name)

    def __eq__(self, other: list | tuple) -> bool:
        if isinstance(other, tuple):
            other = list(other)
        return [self.state, self.max_state] == other
