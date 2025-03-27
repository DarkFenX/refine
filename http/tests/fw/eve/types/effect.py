from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.eve.exception import TestDataConsistencyError
from tests.fw.util import conditional_insert

if typing.TYPE_CHECKING:
    from tests.fw.eve.containers.primitives import EvePrimitives
    from tests.fw.util import Absent
    from .effect_modifier import EffectModifier


@dataclass(kw_only=True)
class Effect:

    id: int
    category_id: int | type[Absent]
    is_assistance: int | bool | type[Absent]
    is_offensive: int | bool | type[Absent]
    discharge_attribute_id: int | type[Absent]
    duration_attribute_id: int | type[Absent]
    range_attribute_id: int | type[Absent]
    falloff_attribute_id: int | type[Absent]
    tracking_attribute_id: int | type[Absent]
    usage_chance_attribute_id: int | type[Absent]
    resist_attribute_id: int | type[Absent]
    modifier_info: list[EffectModifier] | type[Absent]

    def to_primitives(self, *, primitive_data: EvePrimitives) -> None:
        effect_entry = {'effectID': self.id}
        conditional_insert(
            container=effect_entry,
            path=['effectCategory'],
            value=self.category_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['isAssistance'],
            value=self.is_assistance,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['isOffensive'],
            value=self.is_offensive,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['dischargeAttributeID'],
            value=self.discharge_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['durationAttributeID'],
            value=self.duration_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['rangeAttributeID'],
            value=self.range_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['falloffAttributeID'],
            value=self.falloff_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['trackingSpeedAttributeID'],
            value=self.tracking_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['fittingUsageChanceAttributeID'],
            value=self.usage_chance_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['resistanceAttributeID'],
            value=self.resist_attribute_id,
            cast_to=int)
        conditional_insert(
            container=effect_entry,
            path=['modifierInfo'],
            value=(
                [m.to_primitives() for m in self.modifier_info]
                if isinstance(self.modifier_info, list)
                else self.modifier_info))
        if self.id in primitive_data.dogmaeffects:
            msg = f'attempt to add effect with duplicate ID {self.id}'
            raise TestDataConsistencyError(msg)
        primitive_data.dogmaeffects[self.id] = effect_entry
