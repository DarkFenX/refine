from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.eve.exception import TestDataConsistencyError
from tests.fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from tests.fw.util import Absent


@dataclass(kw_only=True)
class Ability:

    id: int
    banned_hisec: int | bool | type[Absent]
    banned_lowsec: int | bool | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        abil_entry = {}
        conditional_insert(container=abil_entry, path=['disallowInHighSec'], value=self.banned_hisec, cast_to=bool)
        conditional_insert(container=abil_entry, path=['disallowInLowSec'], value=self.banned_lowsec, cast_to=bool)
        if self.id in primitive_data.fighterabilities:
            msg = f'attempt to add ability with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.fighterabilities[self.id] = abil_entry
