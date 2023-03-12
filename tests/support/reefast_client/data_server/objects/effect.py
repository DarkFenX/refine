from util import Default
from .aux import conditional_insert


class Effect:

    def __init__(
            self,
            effect_id,
            effect_category=Default,
            is_assistance=Default,
            is_offensive=Default,
            discharge_attribute_id=Default,
            duration_attribute_id=Default,
            range_attribute_id=Default,
            falloff_attribute_id=Default,
            tracking_attribute_id=Default,
            usage_chance_attribute_id=Default,
            resist_attribute_id=Default,
            modifier_info=Default,
    ):
        self.effect_id = effect_id,
        self.effect_category = effect_category
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
        effect_entry = {'effectID': self.effect_id}
        conditional_insert(effect_entry, 'effectCategory', self.effect_category)
        conditional_insert(effect_entry, 'isAssistance', self.is_assistance)
        conditional_insert(effect_entry, 'isOffensive', self.is_offensive)
        conditional_insert(effect_entry, 'dischargeAttributeID', self.discharge_attribute_id)
        conditional_insert(effect_entry, 'durationAttributeID', self.duration_attribute_id)
        conditional_insert(effect_entry, 'rangeAttributeID', self.range_attribute_id)
        conditional_insert(effect_entry, 'falloffAttributeID', self.falloff_attribute_id)
        conditional_insert(effect_entry, 'trackingSpeedAttributeID', self.tracking_attribute_id)
        conditional_insert(effect_entry, 'fittingUsageChanceAttributeID', self.usage_chance_attribute_id)
        conditional_insert(effect_entry, 'resistanceAttributeID', self.resist_attribute_id)
        primitive_data.dogmaeffects[self.effect_id] = effect_entry
