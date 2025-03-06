from dataclasses import dataclass


@dataclass
class ValRigSizeFail:

    allowed_size: float
    mismatches: dict[str, float]

    def __init__(self, *, data: tuple) -> None:
        self.allowed_size = data[0]
        self.mismatches = dict(data[1])

    def __eq__(self, other: tuple) -> bool:
        return (self.allowed_size, self.mismatches) == (other[0], other[1])
