from __future__ import annotations

from typing import TYPE_CHECKING

from tests.support.util import conditional_insert, make_repr_str
from .exception import TestDataConsistencyError

if TYPE_CHECKING:
    from typing import Type, Union

    from tests.support.eve.containers import EvePrimitives
    from tests.support.util import Absent
    from .effect_modifier import EffectModifier


class Effect:

    def __init__(
            self,
            id_: int,
            category_id: Union[int, Type[Absent]],
            is_assistance: Union[int, bool, Type[Absent]],
            is_offensive: Union[int, bool, Type[Absent]],
            discharge_attribute_id: Union[int, Type[Absent]],
            duration_attribute_id: Union[int, Type[Absent]],
            range_attribute_id: Union[int, Type[Absent]],
            falloff_attribute_id: Union[int, Type[Absent]],
            tracking_attribute_id: Union[int, Type[Absent]],
            usage_chance_attribute_id: Union[int, Type[Absent]],
            resist_attribute_id: Union[int, Type[Absent]],
            modifier_info: Union[list[EffectModifier], tuple[EffectModifier], Type[Absent]],
    ):
        self.id = id_
        self.category_id = category_id
        self.is_assistance = is_assistance
        self.is_offensive = is_offensive
        self.discharge_attribute_id = discharge_attribute_id
        self.duration_attribute_id = duration_attribute_id
        self.range_attribute_id = range_attribute_id
        self.falloff_attribute_id = falloff_attribute_id
        self.tracking_attribute_id = tracking_attribute_id
        self.usage_chance_attribute_id = usage_chance_attribute_id
        self.resist_attribute_id = resist_attribute_id
        self.modifier_info = modifier_info

    def to_primitives(self, primitive_data: EvePrimitives) -> None:
        effect_entry = {'effectID': self.id}
        conditional_insert(effect_entry, 'effectCategory', self.category_id, cast_to=int)
        conditional_insert(effect_entry, 'isAssistance', self.is_assistance, cast_to=int)
        conditional_insert(effect_entry, 'isOffensive', self.is_offensive, cast_to=int)
        conditional_insert(effect_entry, 'dischargeAttributeID', self.discharge_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'durationAttributeID', self.duration_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'rangeAttributeID', self.range_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'falloffAttributeID', self.falloff_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'trackingSpeedAttributeID', self.tracking_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'fittingUsageChanceAttributeID', self.usage_chance_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'resistanceAttributeID', self.resist_attribute_id, cast_to=int)
        conditional_insert(effect_entry, 'modifierInfo', (
            [m.to_primitives() for m in self.modifier_info]
            if isinstance(self.modifier_info, (list, tuple))
            else self.modifier_info))
        if self.id in primitive_data.dogmaeffects:
            raise TestDataConsistencyError(f'attempt to add effect with duplicate ID {self.id}')
        primitive_data.dogmaeffects[self.id] = effect_entry

    def __repr__(self) -> str:
        return make_repr_str(self)
