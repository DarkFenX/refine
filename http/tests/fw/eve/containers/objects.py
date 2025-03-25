from __future__ import annotations

import typing

from tests.fw.eve.exception import TestDataConsistencyError
from tests.fw.eve.types import Attribute, Buff, Effect, Group, Item, ItemList, Mutator
from .primitives import EvePrimitives

if typing.TYPE_CHECKING:
    from tests.fw.eve.types import BuffModifier, EffectModifier
    from tests.fw.util import Absent
    from .strings import EveStrings


class EveObjects:

    def __init__(self, *, alias: str) -> None:
        self.alias: str = alias
        # Dictionaries with various entities. Empty list means ID was allocated (but is not
        # presently used).
        self.items: dict[int, list[Item]] = {}
        self.item_groups: dict[int, list[Group]] = {}
        self.item_lists: dict[int, list[ItemList]] = {}
        self.attributes: dict[int, list[Attribute]] = {}
        self.effects: dict[int, list[Effect]] = {}
        self.buffs: dict[int, list[Buff]] = {}
        self.mutators: dict[int, list[Mutator]] = {}
        # Variables point at next ID to allocate
        self.item_id: int = 1000000
        self.item_group_id: int = 2000000
        self.item_list_id: int = 3000000
        self.attr_id: int = 4000000
        self.effect_id: int = 5000000
        self.buff_id: int = 6000000

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

    def prealloc_item_list_id(self) -> int:
        id_ = self.item_list_id
        while id_ in self.item_lists:
            id_ += 1
        return id_

    def alloc_item_list_id(self, *, id_: int) -> None:
        if id_ not in self.item_lists:
            self.item_lists[id_] = []
        if id_ >= self.item_list_id:
            self.item_list_id = id_ + 1

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
            attrs: dict[int, float] | type[Absent],
            eff_ids: list[int] | type[Absent],
            defeff_id: int | None | type[Absent],
            srqs: dict[int, int] | type[Absent],
            capacity: float | type[Absent],
            mass: float | type[Absent],
            radius: float | type[Absent],
            volume: float | type[Absent],
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
            cat_id: int | type[Absent],
    ) -> Group:
        group = Group(id_=id_, category_id=cat_id)
        self.item_groups.setdefault(id_, []).append(group)
        return group

    def mk_item_list(
            self, *,
            id_: int,
            inc_type_ids: list[int] | type[Absent],
            inc_grp_ids: list[int] | type[Absent],
            inc_cat_ids: list[int] | type[Absent],
            exc_type_ids: list[int] | type[Absent],
            exc_grp_ids: list[int] | type[Absent],
            exc_cat_ids: list[int] | type[Absent],
    ) -> ItemList:
        item_list = ItemList(
            id_=id_,
            included_type_ids=inc_type_ids,
            included_group_ids=inc_grp_ids,
            included_category_ids=inc_cat_ids,
            excluded_type_ids=exc_type_ids,
            excluded_group_ids=exc_grp_ids,
            excluded_category_ids=exc_cat_ids)
        self.item_lists.setdefault(id_, []).append(item_list)
        return item_list

    def mk_attr(
            self, *,
            id_: int,
            stackable: int | bool | type[Absent],
            high_is_good: int | bool | type[Absent],
            def_val: float | type[Absent],
            min_attr_id: int | type[Absent],
            max_attr_id: int | type[Absent],
            unit_id: int | type[Absent],
    ) -> Attribute:
        attr = Attribute(
            id_=id_,
            stackable=stackable,
            high_is_good=high_is_good,
            default_value=def_val,
            min_attribute_id=min_attr_id,
            max_attribute_id=max_attr_id,
            unit_id=unit_id)
        self.attributes.setdefault(id_, []).append(attr)
        return attr

    def mk_effect(
            self, *,
            id_: int,
            cat_id: int | type[Absent],
            is_assistance: int | bool | type[Absent],
            is_offensive: int | bool | type[Absent],
            discharge_attr_id: int | type[Absent],
            duration_attr_id: int | type[Absent],
            range_attr_id: int | type[Absent],
            falloff_attr_id: int | type[Absent],
            tracking_attr_id: int | type[Absent],
            chance_attr_id: int | type[Absent],
            resist_attr_id: int | type[Absent],
            mod_info: list[EffectModifier] | type[Absent],
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
            aggr_mode: str | type[Absent],
            op: str | type[Absent],
            item_mods: list[BuffModifier] | type[Absent],
            loc_mods: list[BuffModifier] | type[Absent],
            loc_grp_mods: list[BuffModifier] | type[Absent],
            loc_srq_mods: list[BuffModifier] | type[Absent],
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
            items: list[tuple[list[int], int]] | type[Absent],
            attrs: dict[int, tuple[float, float]] | type[Absent],
    ) -> Mutator:
        mutator = Mutator(
            id_=id_,
            items=items,
            attributes=attrs)
        self.mutators.setdefault(id_, []).append(mutator)
        return mutator

    def render(self) -> EveStrings:
        return self.to_primitives().to_strings()

    def to_primitives(self) -> EvePrimitives:
        primitive_data = EvePrimitives(alias=self.alias)
        self.__handle_container(primitive_data=primitive_data, container=self.items, entity_class=Item)
        self.__handle_container(primitive_data=primitive_data, container=self.item_groups, entity_class=Group)
        self.__handle_container(primitive_data=primitive_data, container=self.item_lists, entity_class=ItemList)
        self.__handle_container(primitive_data=primitive_data, container=self.attributes, entity_class=Attribute)
        self.__handle_container(primitive_data=primitive_data, container=self.effects, entity_class=Effect)
        self.__handle_container(primitive_data=primitive_data, container=self.buffs, entity_class=Buff)
        self.__handle_container(primitive_data=primitive_data, container=self.mutators, entity_class=Mutator)
        return primitive_data

    @staticmethod
    def __handle_container(*, primitive_data: EvePrimitives, container: dict[int, list], entity_class: object) -> None:
        for entity_list in container.values():
            if len(entity_list) > 1:
                msg = f'expected 1 {entity_class.__name__}, got {len(entity_list)}'
                raise TestDataConsistencyError(msg)
            for entity in entity_list:
                entity.to_primitives(primitive_data=primitive_data)
