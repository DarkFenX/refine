from dataclasses import dataclass


@dataclass
class ValRigSizeFail:

    allowed_size: float
    rig_sizes: dict[str, float | None]

    def __init__(self, *, data: tuple) -> None:
        self.allowed_size = data[0]
        self.rig_sizes = data[1]

    def __eq__(self, other: tuple) -> bool:
        return (self.allowed_size, self.rig_sizes) == (other[0], other[1])
