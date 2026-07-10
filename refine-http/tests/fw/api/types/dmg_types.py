from dataclasses import dataclass


@dataclass(kw_only=True)
class DmgTypes:

    em: float
    thermal: float
    kinetic: float
    explosive: float
