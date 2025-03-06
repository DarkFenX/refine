from dataclasses import dataclass


class ValModuleStateFail(dict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({k: ValModuleStateInfo(data=v) for k, v in data.items()})


@dataclass
class ValModuleStateInfo:

    state: str
    max_state: str

    def __init__(self, *, data: tuple) -> None:
        self.state = data[0]
        self.max_state = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.state, self.max_state) == (other[0], other[1])
