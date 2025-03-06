from dataclasses import dataclass


@dataclass
class ValCapModuleFail:

    max_subcap_volume: float
    module_volumes: dict[str, float]

    def __init__(self, *, data: tuple) -> None:
        self.max_subcap_volume = data[0]
        self.module_volumes = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.max_subcap_volume, self.module_volumes) == (other[0], other[1])
