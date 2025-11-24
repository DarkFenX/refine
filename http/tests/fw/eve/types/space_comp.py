import typing
from dataclasses import dataclass

from tests.fw.util import Absent, conditional_insert

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives


@dataclass(kw_only=True)
class SpaceComponent:

    type_id: int
    system_wide_buffs: SpaceComponentBuffData | type[Absent]
    system_emitter_buffs: SpaceComponentBuffData | type[Absent]
    proxy_effect_buffs: SpaceComponentBuffData | type[Absent]
    proxy_trap_buffs: SpaceComponentBuffData | type[Absent]
    ship_link_buffs: SpaceComponentBuffData | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        space_comp_entry = {}
        # System-wide effects
        if self.system_wide_buffs is not Absent:
            conditional_insert(
                container=space_comp_entry,
                path=['systemWideEffects', 'globalDebuffs', 'dbuffs'],
                value=self.system_wide_buffs.buffs)
            conditional_insert(
                container=space_comp_entry,
                path=['systemWideEffects', 'globalDebuffs', 'eligibleTypeListID'],
                value=self.system_wide_buffs.item_list_id)
        # System buff emitters
        if self.system_emitter_buffs is not Absent:
            conditional_insert(
                container=space_comp_entry,
                path=['systemDbuffEmitter', 'dbuffCollections'],
                value=self.system_emitter_buffs.buffs)
        # Proximity effects
        if self.proxy_effect_buffs is not Absent:
            conditional_insert(
                container=space_comp_entry,
                path=['appliedProximityEffects', 'effects'],
                value=self.proxy_effect_buffs.buffs)
        # Proximity traps
        if self.proxy_trap_buffs is not Absent:
            conditional_insert(
                container=space_comp_entry,
                path=['proximityTrap', 'dbuffs'],
                value=self.proxy_trap_buffs.buffs)
            conditional_insert(
                container=space_comp_entry,
                path=['proximityTrap', 'triggerFilterTypeListID'],
                value=self.proxy_trap_buffs.item_list_id)
        # Ship links
        if self.ship_link_buffs is not Absent:
            conditional_insert(
                container=space_comp_entry,
                path=['linkWithShip', 'dbuffs'],
                value=self.ship_link_buffs.buffs)
            conditional_insert(
                container=space_comp_entry,
                path=['linkWithShip', 'linkableShipTypeListID'],
                value=self.ship_link_buffs.item_list_id)
        primitive_data.spacecomponentsbytype[self.type_id] = space_comp_entry


@dataclass(kw_only=True)
class SpaceComponentBuffData:

    buffs: dict[int, float]
    item_list_id: int | type[Absent] = Absent

    @classmethod
    def from_raw(cls, data: dict[int, float] | tuple[dict[int, float], int]) -> SpaceComponentBuffData:
        if isinstance(data, tuple | list):
            buffs, item_list_id = data
            return SpaceComponentBuffData(buffs=buffs, item_list_id=item_list_id)
        return SpaceComponentBuffData(buffs=data)
