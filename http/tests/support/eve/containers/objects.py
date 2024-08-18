from __future__ import annotations

from collections import defaultdict
from itertools import chain
from typing import TYPE_CHECKING

from tests.support.consts import EveEffCat, EveEffect, EveItemCat
from tests.support.eve.types import Item, Group, Attribute, Effect, Buff
from tests.support.util import Absent, Default
from .primitives import EvePrimitives

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.eve.types import BuffModifier, EffectModifier
    from .strings import EveStrings


ID_START = 1000000


class EveObjects:

    def __init__(self, alias: str):
        self.alias = alias
        self.items = []
        self.item_groups = []
        self.attributes = []
        self.effects = []
        self.buffs = []
        self.item_id = ID_START
        self.item_group_id = ID_START
        self.attr_id = ID_START
        self.effect_id = ID_START
        self.buff_id = ID_START
        # Helper map to ease management of groups/categories
        # Format: {group ID: [groups]}
        self.group_map = defaultdict(lambda: [])

    def alloc_item_id(self, avoid_ids: Union[tuple[int], list[int]] = ()) -> int:
        while self.__is_item_id_used(self.item_id, avoid_ids=avoid_ids):
            self.item_id += 1
        id_ = self.item_id
        self.item_id += 1
        return id_

    def __is_item_id_used(self, id_: int, avoid_ids: Union[tuple[int], list[int]]) -> bool:
        return id_ in chain((i.id for i in self.items), avoid_ids)

    def alloc_group_id(self, avoid_ids: Union[tuple[int], list[int]] = ()) -> int:
        while self.__is_group_id_used(self.item_group_id, avoid_ids=avoid_ids):
            self.item_group_id += 1
        id_ = self.item_group_id
        self.item_group_id += 1
        return id_

    def __is_group_id_used(self, id_: int, avoid_ids: Union[tuple[int], list[int]]) -> bool:
        return id_ in chain((ig.id for ig in self.item_groups), avoid_ids)

    def alloc_attr_id(self, avoid_ids: Union[tuple[int], list[int]] = ()) -> int:
        while self.__is_attr_id_used(self.attr_id, avoid_ids=avoid_ids):
            self.attr_id += 1
        id_ = self.attr_id
        self.attr_id += 1
        return id_

    def __is_attr_id_used(self, id_: int, avoid_ids: Union[tuple[int], list[int]]) -> bool:
        return id_ in chain((a.id for a in self.attributes), avoid_ids)

    def alloc_effect_id(self, avoid_ids: Union[tuple[int], list[int]] = ()) -> int:
        while self.__is_effect_id_used(self.effect_id, avoid_ids=avoid_ids):
            self.effect_id += 1
        id_ = self.effect_id
        self.effect_id += 1
        return id_

    def __is_effect_id_used(self, id_: int, avoid_ids: Union[tuple[int], list[int]]) -> bool:
        return id_ in chain((e.id for e in self.effects), avoid_ids)

    def alloc_buff_id(self, avoid_ids: Union[tuple[int], list[int]] = ()) -> int:
        while self.__is_buff_id_used(self.buff_id, avoid_ids=avoid_ids):
            self.buff_id += 1
        id_ = self.buff_id
        self.buff_id += 1
        return id_

    def __is_buff_id_used(self, id_: int, avoid_ids: Union[tuple[int], list[int]]) -> bool:
        return id_ in chain((b.id for b in self.buffs), avoid_ids)

    def mk_item(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            grp_id: Union[int, Type[Default]] = Default,
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
            attrs: Union[dict[int, float], Type[Absent], Type[Default]] = Default,
            eff_ids: Union[list[int], tuple[int], Type[Absent], Type[Default]] = Default,
            defeff_id: Union[int, None, Type[Absent], Type[Default]] = Default,
            srqs: Union[dict[int, int], Type[Absent], Type[Default]] = Default,
            capacity: Union[float, Type[Absent], Type[Default]] = Default,
            mass: Union[float, Type[Absent], Type[Default]] = Default,
            radius: Union[float, Type[Absent], Type[Default]] = Default,
            volume: Union[float, Type[Absent], Type[Default]] = Default,
    ) -> Item:
        if id_ is Default:
            id_ = self.alloc_item_id(avoid_ids=avoid_ids)
        group = self.__fetch_or_mk_item_group(id_=grp_id, avoid_ids=avoid_ids, cat_id=cat_id)
        item = Item(
            id_=id_,
            group_id=group.id,
            attributes={} if attrs is Default else attrs,
            effect_ids=[] if eff_ids is Default else eff_ids,
            default_effect_id=None if defeff_id is Default else defeff_id,
            skill_reqs={} if srqs is Default else srqs,
            capacity=0.0 if capacity is Default else capacity,
            mass=0.0 if mass is Default else mass,
            radius=0.0 if radius is Default else radius,
            volume=0.0 if volume is Default else volume)
        self.items.append(item)
        return item

    def mk_ship(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            grp_id: Union[int, Type[Default]] = Default,
            attrs: Union[dict[int, float], Type[Absent], Type[Default]] = Default,
            eff_ids: Union[list[int], tuple[int], Type[Absent], Type[Default]] = Default,
            defeff_id: Union[int, None, Type[Absent], Type[Default]] = Default,
            srqs: Union[dict[int, int], Type[Absent], Type[Default]] = Default,
            capacity: Union[float, Type[Absent], Type[Default]] = Default,
            mass: Union[float, Type[Absent], Type[Default]] = Default,
            radius: Union[float, Type[Absent], Type[Default]] = Default,
            volume: Union[float, Type[Absent], Type[Default]] = Default,
    ) -> Item:
        return self.mk_item(
            id_=id_,
            avoid_ids=avoid_ids,
            grp_id=grp_id,
            cat_id=EveItemCat.ship,
            attrs=attrs,
            eff_ids=eff_ids,
            defeff_id=defeff_id,
            srqs=srqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)

    def mk_struct(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            grp_id: Union[int, Type[Default]] = Default,
            attrs: Union[dict[int, float], Type[Absent], Type[Default]] = Default,
            eff_ids: Union[list[int], tuple[int], Type[Absent], Type[Default]] = Default,
            defeff_id: Union[int, None, Type[Absent], Type[Default]] = Default,
            srqs: Union[dict[int, int], Type[Absent], Type[Default]] = Default,
            capacity: Union[float, Type[Absent], Type[Default]] = Default,
            mass: Union[float, Type[Absent], Type[Default]] = Default,
            radius: Union[float, Type[Absent], Type[Default]] = Default,
            volume: Union[float, Type[Absent], Type[Default]] = Default,
    ) -> Item:
        return self.mk_item(
            id_=id_,
            avoid_ids=avoid_ids,
            grp_id=grp_id,
            cat_id=EveItemCat.structure,
            attrs=attrs,
            eff_ids=eff_ids,
            defeff_id=defeff_id,
            srqs=srqs,
            capacity=capacity,
            mass=mass,
            radius=radius,
            volume=volume)

    def mk_item_group(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
    ) -> Group:
        if id_ is Default:
            id_ = self.alloc_group_id(avoid_ids=avoid_ids)
        if cat_id is Default:
            cat_id = EveItemCat.module
        group = Group(id_=id_, category_id=cat_id)
        self.item_groups.append(group)
        self.group_map[group.id].append(group)
        return group

    def mk_ship_group(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
    ) -> Group:
        return self.mk_item_group(id_=id_, avoid_ids=avoid_ids, cat_id=EveItemCat.ship)

    def mk_struct_group(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
    ) -> Group:
        return self.mk_item_group(id_=id_, avoid_ids=avoid_ids, cat_id=EveItemCat.structure)

    def __fetch_or_mk_item_group(
            self,
            id_: Union[int, Type[Default]],
            avoid_ids: Union[tuple[int], list[int]],
            cat_id: Union[int, Type[Absent], Type[Default]],
    ) -> Group:
        # Fetch existing group if consistency is not broken:
        # - when requested category ID and group's category ID match
        # - when default category ID is requested, and we have just one group
        if id_ in self.group_map:
            groups = self.group_map[id_]
            if len(groups) == 1:
                group = groups[0]
                if cat_id is Default or cat_id == group.category_id:
                    return group
        return self.mk_item_group(id_=id_, avoid_ids=avoid_ids, cat_id=cat_id)

    def mk_attr(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            stackable: Union[int, bool, Type[Absent], Type[Default]] = Default,
            high_is_good: Union[int, bool, Type[Absent], Type[Default]] = Default,
            def_val: Union[float, Type[Absent], Type[Default]] = Default,
            max_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
    ) -> Attribute:
        if id_ is Default:
            id_ = self.alloc_attr_id(avoid_ids=avoid_ids)
        attr = Attribute(
            id_=id_,
            stackable=1 if stackable is Default else stackable,
            high_is_good=1 if high_is_good is Default else high_is_good,
            default_value=0.0 if def_val is Default else def_val,
            max_attribute_id=Absent if max_attr_id is Default else max_attr_id)
        self.attributes.append(attr)
        return attr

    def mk_effect(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            cat_id: Union[int, Type[Absent], Type[Default]] = Default,
            is_assistance: Union[int, bool, Type[Absent], Type[Default]] = Default,
            is_offensive: Union[int, bool, Type[Absent], Type[Default]] = Default,
            discharge_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            duration_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            range_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            falloff_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            tracking_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            chance_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            resist_attr_id: Union[int, Type[Absent], Type[Default]] = Default,
            mod_info: Union[list[EffectModifier], tuple[EffectModifier], Type[Absent], Type[Default]] = Default,
    ) -> Effect:
        if id_ is Default:
            id_ = self.alloc_effect_id(avoid_ids=avoid_ids)
        effect = Effect(
            id_=id_,
            category_id=EveEffCat.passive if cat_id is Default else cat_id,
            is_assistance=0 if is_assistance is Default else is_assistance,
            is_offensive=0 if is_offensive is Default else is_offensive,
            discharge_attribute_id=Absent if discharge_attr_id is Default else discharge_attr_id,
            duration_attribute_id=Absent if duration_attr_id is Default else duration_attr_id,
            range_attribute_id=Absent if range_attr_id is Default else range_attr_id,
            falloff_attribute_id=Absent if falloff_attr_id is Default else falloff_attr_id,
            tracking_attribute_id=Absent if tracking_attr_id is Default else tracking_attr_id,
            usage_chance_attribute_id=Absent if chance_attr_id is Default else chance_attr_id,
            resist_attribute_id=Absent if resist_attr_id is Default else resist_attr_id,
            modifier_info=Absent if mod_info is Default else mod_info)
        self.effects.append(effect)
        return effect

    def mk_online_effect(self) -> Effect:
        return self.mk_effect(
            id_=EveEffect.online,
            cat_id=EveEffCat.active,
            is_assistance=0,
            is_offensive=0,
            discharge_attr_id=Absent,
            duration_attr_id=Absent,
            range_attr_id=Absent,
            falloff_attr_id=Absent,
            tracking_attr_id=Absent,
            chance_attr_id=Absent,
            resist_attr_id=Absent,
            mod_info=Absent)

    def mk_buff(
            self,
            id_: Union[int, Type[Default]] = Default,
            avoid_ids: Union[tuple[int], list[int]] = (),
            aggr_mode: Union[str, Type[Absent], Type[Default]] = Default,
            op: Union[str, Type[Absent], Type[Default]] = Default,
            item_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_grp_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
            loc_srq_mods: Union[list[BuffModifier], tuple[BuffModifier], Type[Absent], Type[Default]] = Default,
    ) -> Buff:
        if id_ is Default:
            id_ = self.alloc_buff_id(avoid_ids=avoid_ids)
        buff = Buff(
            id_=id_,
            aggregate_mode=Absent if aggr_mode is Default else aggr_mode,
            operation_name=Absent if op is Default else op,
            item_modifiers=Absent if item_mods is Default else item_mods,
            location_modifiers=Absent if loc_mods is Default else loc_mods,
            location_group_modifiers=Absent if loc_grp_mods is Default else loc_grp_mods,
            location_skillreq_modifiers=Absent if loc_srq_mods is Default else loc_srq_mods)
        self.buffs.append(buff)
        return buff

    def render(self) -> EveStrings:
        primitives = self.to_primitives()
        strings = primitives.to_strings()
        return strings

    def to_primitives(self) -> EvePrimitives:
        primitive_data = EvePrimitives(self.alias)
        for item in self.items:
            item.to_primitives(primitive_data)
        for item_group in self.item_groups:
            item_group.to_primitives(primitive_data)
        for attr in self.attributes:
            attr.to_primitives(primitive_data)
        for effect in self.effects:
            effect.to_primitives(primitive_data)
        for buff in self.buffs:
            buff.to_primitives(primitive_data)
        return primitive_data
