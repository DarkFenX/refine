from tests.support.util import Default, Absent
from .aux import TestDataConsistencyError, conditional_insert


class Item:

    def __init__(
            self,
            id_,
            group_id,
            category_id,
            attributes,
            effect_ids,
            default_effect_id,
            skill_reqs,
    ):
        self.id = id_
        self.group_id = group_id
        self.category_id = category_id
        self.attributes = attributes
        self.effect_ids = effect_ids
        self.default_effect_id = default_effect_id
        self.skill_reqs = skill_reqs

    def to_primitives(self, primitive_data):
        item_entry = {'typeID': self.id}
        conditional_insert(item_entry, 'groupID', self.group_id)
        primitive_data.types[self.id] = item_entry
        self.__add_primitive_group(primitive_data)
        self.__add_primitive_attributes(primitive_data)
        self.__add_primitive_effects(primitive_data)
        conditional_insert(primitive_data.requiredskillsfortypes, self.id, self.skill_reqs)

    def __add_primitive_group(self, primitive_data):
        if self.group_id is Absent:
            return
        if self.group_id in primitive_data.groups:
            group_entry = primitive_data.groups[self.group_id]
            if ((self.category_id is Absent and 'categoryID' in group_entry) or
                    (self.category_id is not Absent and group_entry.get('categoryID', Absent) != self.category_id)):
                raise TestDataConsistencyError('attempt to add group which already exists and has different category')
        group_entry = primitive_data.groups.setdefault(self.group_id, {'groupID': self.group_id})
        conditional_insert(group_entry, 'categoryID', self.category_id)

    def __add_primitive_attributes(self, primitive_data):
        if self.attributes is Default:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.attributes, dict):
            attrs_entry = item_entry['dogmaAttributes'] = []
            for attr_id, attr_val in self.attributes.items():
                attrs_entry.append({'attributeID': attr_id, 'value': attr_val})
        else:
            item_entry['dogmaAttributes'] = self.attributes

    def __add_primitive_effects(self, primitive_data):
        if self.effect_ids is Default:
            return
        item_entry = primitive_data.typedogma.setdefault(self.id, {})
        if isinstance(self.effect_ids, (tuple, list, set)):
            effects_entry = item_entry['dogmaEffects'] = []
            for effect_id in self.effect_ids:
                effects_entry.append({'effectID': effect_id, 'isDefault': int(effect_id == self.default_effect_id)})
        else:
            item_entry['dogmaEffects'] = self.effect_ids
