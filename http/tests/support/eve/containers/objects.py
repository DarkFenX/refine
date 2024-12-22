from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.eve.types import Attribute, Buff, Effect, Group, Item, Mutator
from tests.support.util import Absent
from .primitives import EvePrimitives

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.eve.types import BuffModifier, EffectModifier
    from .strings import EveStrings


ID_START = 1000000


class EveObjects:

    def __init__(self, *, alias: str):
        self.alias: str = alias
        # Dictionaries with various entities. Empty list means ID was allocated (but is not
        # presently used).
        self.items: dict[int, list[Item]] = {}
        self.item_groups: dict[int, list[Group]] = {}
        self.attributes: dict[int, list[Attribute]] = {}
        self.effects: dict[int, list[Effect]] = {}
        self.buffs: dict[int, list[Buff]] = {}
        self.mutators: dict[int, list[Mutator]] = {}
        # Variables point at next ID to allocate
        self.item_id: int = ID_START
        self.item_group_id: int = ID_START
        self.attr_id: int = ID_START
        self.effect_id: int = ID_START
        self.buff_id: int = ID_START

    def prealloc_item_id(self) -> int:
        id_ = self.item_id
        while id_ in self.items:
            id_ += 1
        return id_

    def alloc_item_id(self, *, id_: int) -> None:
        if id_ not in self.items:
            self.items[id_] = []
        if id_ >= self.item_id:
            self.item_id = id_ + 1

    def prealloc_group_id(self) -> int:
        id_ = self.item_group_id
        while id_ in self.item_groups:
            id_ += 1
        return id_

    def alloc_group_id(self, *, id_: int) -> None:
        if id_ not in self.item_groups:
            self.item_groups[id_] = []
        if id_ >= self.item_group_id:
            self.item_group_id = id_ + 1

    def prealloc_attr_id(self) -> int:
        id_ = self.attr_id
        while id_ in self.attributes:
            id_ += 1
        return id_

    def alloc_attr_id(self, *, id_: int) -> None:
        if id_ not in self.attributes:
            self.attributes[id_] = []
        if id_ >= self.attr_id:
            self.attr_id = id_ + 1

    def prealloc_effect_id(self) -> int:
        id_ = self.effect_id
        while id_ in self.effects:
            id_ += 1
        return id_

    def alloc_effect_id(self, *, id_: int) -> None:
        if id_ not in self.effects:
            self.effects[id_] = []
        if id_ >= self.effect_id:
            self.effect_id = id_ + 1

    def prealloc_buff_id(self) -> int:
        id_ = self.buff_id
        while id_ in self.buffs:
            id_ += 1
        return id_

    def alloc_buff_id(self, *, id_: int) -> None:
        if id_ not in self.buffs:
            self.buffs[id_] = []
        if id_ >= self.buff_id:
            self.buff_id = id_ + 1

    def mk_item(
            self, *,
            id_: int,
            grp_id: int,
            attrs: Union[dict[int, float], Type[Absent]],
            eff_ids: Union[list[int], Type[Absent]],
            defeff_id: Union[int, None, Type[Absent]],
            srqs: Union[dict[int, int], Type[Absent]],
            capacity: Union[float, Type[Absent]],
            mass: Union[float, Type[Absent]],
            radius: Union[float, Type[Absent]],
            volume: Union[float, Type[Absent]],
    ) -> Item:
        item = Item(
            id_=id_,
            group_id=grp_id,
            attributes=attrs,
            effect_ids=eff_ids,
            default_effect_id=defeff_id,
            skill_reqs=srqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)
        self.items.setdefault(id_, []).append(item)
        return item

    def mk_item_group(
            self, *,
            id_: int,
            cat_id: Union[int, Type[Absent]],
    ) -> Group:
        group = Group(id_=id_, category_id=cat_id)
        self.item_groups.setdefault(id_, []).append(group)
        return group

    def mk_attr(
            self, *,
            id_: int,
            stackable: Union[int, bool, Type[Absent]],
            high_is_good: Union[int, bool, Type[Absent]],
            def_val: Union[float, Type[Absent]],
            min_attr_id: Union[int, Type[Absent]],
            max_attr_id: Union[int, Type[Absent]],
    ) -> Attribute:
        attr = Attribute(
            id_=id_,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            min_attribute_id=min_attr_id,
            max_attribute_id=max_attr_id)
        self.attributes.setdefault(id_, []).append(attr)
        return attr

    def mk_effect(
            self, *,
            id_: int,
            cat_id: Union[int, Type[Absent]],
            is_assistance: Union[int, bool, Type[Absent]],
            is_offensive: Union[int, bool, Type[Absent]],
            discharge_attr_id: Union[int, Type[Absent]],
            duration_attr_id: Union[int, Type[Absent]],
            range_attr_id: Union[int, Type[Absent]],
            falloff_attr_id: Union[int, Type[Absent]],
            tracking_attr_id: Union[int, Type[Absent]],
            chance_attr_id: Union[int, Type[Absent]],
            resist_attr_id: Union[int, Type[Absent]],
            mod_info: Union[list[EffectModifier], Type[Absent]],
    ) -> Effect:
        effect = Effect(
            id_=id_,
            category_id=cat_id,
            is_assistance=is_assistance,
            is_offensive=is_offensive,
            discharge_attribute_id=discharge_attr_id,
            duration_attribute_id=duration_attr_id,
            range_attribute_id=range_attr_id,
            falloff_attribute_id=falloff_attr_id,
            tracking_attribute_id=tracking_attr_id,
            usage_chance_attribute_id=chance_attr_id,
            resist_attribute_id=resist_attr_id,
            modifier_info=mod_info)
        self.effects.setdefault(id_, []).append(effect)
        return effect

    def mk_buff(
            self, *,
            id_: int,
            aggr_mode: Union[str, Type[Absent]],
            op: Union[str, Type[Absent]],
            item_mods: Union[list[BuffModifier], Type[Absent]],
            loc_mods: Union[list[BuffModifier], Type[Absent]],
            loc_grp_mods: Union[list[BuffModifier], Type[Absent]],
            loc_srq_mods: Union[list[BuffModifier], Type[Absent]],
    ) -> Buff:
        buff = Buff(
            id_=id_,
            aggregate_mode=aggr_mode,
            operation_name=op,
            item_modifiers=item_mods,
            location_modifiers=loc_mods,
            location_group_modifiers=loc_grp_mods,
            location_skillreq_modifiers=loc_srq_mods)
        self.buffs.setdefault(id_, []).append(buff)
        return buff

    def mk_mutator(
            self, *,
            id_: int,
            items: Union[list[tuple[list[int], int]], Type[Absent]],
            attrs: Union[dict[int, tuple[float, float]], Type[Absent]],
    ) -> Mutator:
        mutator = Mutator(
            id_=id_,
            items=items,
            attributes=attrs)
        self.mutators.setdefault(id_, []).append(mutator)
        return mutator

    def render(self) -> EveStrings:
        primitives = self.to_primitives()
        strings = primitives.to_strings()
        return strings

    def to_primitives(self) -> EvePrimitives:
        primitive_data = EvePrimitives(alias=self.alias)
        for item_list in self.items.values():
            for item in item_list:
                item.to_primitives(primitive_data=primitive_data)
        for item_group_list in self.item_groups.values():
            for item_group in item_group_list:
                item_group.to_primitives(primitive_data=primitive_data)
        for attr_list in self.attributes.values():
            for attr in attr_list:
                attr.to_primitives(primitive_data=primitive_data)
        for effect_list in self.effects.values():
            for effect in effect_list:
                effect.to_primitives(primitive_data=primitive_data)
        for buff_list in self.buffs.values():
            for buff in buff_list:
                buff.to_primitives(primitive_data=primitive_data)
        for mutator_list in self.mutators.values():
            for mutator in mutator_list:
                mutator.to_primitives(primitive_data=primitive_data)
        return primitive_data
