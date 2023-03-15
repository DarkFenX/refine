from .aux import conditional_insert


class Attribute:

    def __init__(
            self,
            id,
            stackable,
            high_is_good,
            default_value,
            max_attribute_id
    ):
        self.id = id
        self.stackable = stackable
        self.high_is_good = high_is_good
        self.default_value = default_value
        self.max_attribute_id = max_attribute_id

    def to_primitives(self, primitive_data):
        attr_entry = {'attributeID': self.id}
        conditional_insert(attr_entry, 'stackable', self.stackable)
        conditional_insert(attr_entry, 'highIsGood', self.high_is_good)
        conditional_insert(attr_entry, 'defaultValue', self.default_value)
        conditional_insert(attr_entry, 'maxAttributeID', self.max_attribute_id)
        primitive_data.dogmaattributes[self.id] = attr_entry
