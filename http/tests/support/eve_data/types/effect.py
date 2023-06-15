from tests.support.util import conditional_insert
from .exception import TestDataConsistencyError


class Effect:

    def __init__(
            self,
            id_,
            category_id,
            is_assistance,
            is_offensive,
            discharge_attribute_id,
            duration_attribute_id,
            range_attribute_id,
            falloff_attribute_id,
            tracking_attribute_id,
            usage_chance_attribute_id,
            resist_attribute_id,
            modifier_info,
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

    def to_primitives(self, primitive_data):
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
        if isinstance(self.modifier_info, (list, tuple)):
            modifier_info = [m.to_primitives() for m in self.modifier_info]
        else:
            modifier_info = self.modifier_info
        conditional_insert(effect_entry, 'modifierInfo', modifier_info)
        if self.id in primitive_data.dogmaeffects:
            raise TestDataConsistencyError(f'attempt to add effect with duplicate ID {self.id}')
        primitive_data.dogmaeffects[self.id] = effect_entry
