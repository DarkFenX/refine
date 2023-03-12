from util import Default
from .aux import conditional_insert


class Attribute:

    def __init__(
            self,
            attr_id,
            stackable=Default,
            high_is_good=Default,
            default_value=Default,
            max_attribute_id=Default
    ):
        self.attr_id = attr_id
        self.stackable = stackable
        self.high_is_good = high_is_good
        self.default_value = default_value
        self.max_attribute_id = max_attribute_id

    def to_primitives(self, primitive_data):
        attr_entry = {'attributeID': self.attr_id}
        conditional_insert(attr_entry, 'stackable', self.stackable)
        conditional_insert(attr_entry, 'highIsGood', self.high_is_good)
        conditional_insert(attr_entry, 'defaultValue', self.default_value)
        conditional_insert(attr_entry, 'maxAttributeID', self.max_attribute_id)
        primitive_data.dogmaattributes[self.attr_id] = attr_entry
