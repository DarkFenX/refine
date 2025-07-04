import dataclasses


@dataclasses.dataclass(kw_only=True)
class AttrVals:

    base: float
    dogma: float
    extra: float
