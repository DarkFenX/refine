import dataclasses
import typing

from fw.util import Absent


@dataclasses.dataclass(kw_only=True)
class ModAddMode:

    mode: str
    pos: float | type[Absent] = Absent

    @classmethod
    def append(cls) -> typing.Self:
        return cls(mode='append')

    @classmethod
    def equip(cls) -> typing.Self:
        return cls(mode='equip')

    @classmethod
    def insert(cls, *, pos: float) -> typing.Self:
        return cls(mode='insert', pos=pos)

    @classmethod
    def replace(cls, *, pos: float) -> typing.Self:
        return cls(mode='replace', pos=pos)


@dataclasses.dataclass(kw_only=True)
class ModMoveMode:

    mode: str
    pos: float | type[Absent] = Absent

    @classmethod
    def shift(cls, *, pos: float) -> typing.Self:
        return cls(mode='shift', pos=pos)

    @classmethod
    def swap(cls, *, pos: float) -> typing.Self:
        return cls(mode='swap', pos=pos)
