from __future__ import annotations

import typing

from tests.fw.util import conditional_insert, make_repr_str

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from tests.fw.util import Absent


class SpaceComponent:

    def __init__(
            self, *,
            type_id: int,
            system_emitter_buffs: dict[int, float] | type[Absent],
            proxy_effect_buffs: dict[int, float] | type[Absent],
            proxy_trigger_buffs: dict[int, float] | type[Absent],
            ship_link_buffs: dict[int, float] | type[Absent],
    ) -> None:
        self.type_id = type_id
        self.system_emitter_buffs = system_emitter_buffs
        self.proxy_effect_buffs = proxy_effect_buffs
        self.proxy_trigger_buffs = proxy_trigger_buffs
        self.ship_link_buffs = ship_link_buffs

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        space_comp_entry = {}
        conditional_insert(
            container=space_comp_entry,
            path=['systemDbuffEmitter', 'dbuffCollections'],
            value=self.system_emitter_buffs)
        conditional_insert(
            container=space_comp_entry,
            path=['appliedProximityEffects', 'effects'],
            value=self.proxy_effect_buffs)
        conditional_insert(
            container=space_comp_entry,
            path=['proximityTrap', 'dbuffs'],
            value=self.proxy_effect_buffs)
        conditional_insert(
            container=space_comp_entry,
            path=['linkWithShip', 'dbuffs'],
            value=self.proxy_effect_buffs)
        primitive_data.typelist[self.type_id] = space_comp_entry

    def __repr__(self) -> str:
        return make_repr_str(instance=self)
