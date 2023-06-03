import json

from tests.support.util import Default
from .types import Item, Attribute, Effect, Buff

ID_START = 1000000


class TestObjects:

    def __init__(self, alias):
        self.alias = alias
        self.items = []
        self.attributes = []
        self.effects = []
        self.buffs = []
        self.item_id = ID_START
        self.item_group_id = ID_START
        self.attr_id = ID_START
        self.effect_id = ID_START
        self.buff_id = ID_START

    def mk_item(self, id_, group_id, category_id, *args, **kwargs):
        if id_ is Default:
            id_ = self.item_id
            self.item_id += 1
        if group_id is Default:
            group_id = self.item_group_id
            self.item_group_id += 1
        item = Item(id_=id_, group_id=group_id, category_id=category_id, *args, **kwargs)
        self.items.append(item)
        return item

    def mk_attr(self, id_, *args, **kwargs):
        if id_ is Default:
            id_ = self.attr_id
            self.attr_id += 1
        attr = Attribute(id_, *args, **kwargs)
        self.attributes.append(attr)
        return attr

    def mk_effect(self, id_, *args, **kwargs):
        if id_ is Default:
            id_ = self.effect_id
            self.effect_id += 1
        effect = Effect(id_, *args, **kwargs)
        self.effects.append(effect)
        return effect

    def mk_buff(self, id_, *args, **kwargs):
        if id_ is Default:
            id_ = self.buff_id
            self.buff_id += 1
        buff = Buff(id_, *args, **kwargs)
        self.buffs.append(buff)
        return buff

    def render(self):
        primitives = self.to_primitives()
        strings = primitives.to_strings()
        return strings

    def to_primitives(self):
        primitive_data = TestPrimitives(self.alias)
        for item in self.items:
            item.to_primitives(primitive_data)
        for attr in self.attributes:
            attr.to_primitives(primitive_data)
        for effect in self.effects:
            effect.to_primitives(primitive_data)
        for buff in self.buffs:
            buff.to_primitives(primitive_data)
        return primitive_data


class TestPrimitives:

    def __init__(self, alias):
        self.alias = alias
        self.types = {}
        self.groups = {}
        self.typedogma = {}
        self.dogmaattributes = {}
        self.dogmaeffects = {}
        self.fighterabilities = {}
        self.fighterabilitiesbytype = {}
        self.dbuffcollections = {}
        self.requiredskillsfortypes = {}
        self.dynamicitemattributes = {}

    def to_strings(self):
        string_data = TestStrings(self.alias)
        string_data.types = json.dumps(self.types)
        string_data.groups = json.dumps(self.groups)
        string_data.typedogma = json.dumps(self.typedogma)
        string_data.dogmaattributes = json.dumps(self.dogmaattributes)
        string_data.dogmaeffects = json.dumps(self.dogmaeffects)
        string_data.fighterabilities = json.dumps(self.fighterabilities)
        string_data.fighterabilitiesbytype = json.dumps(self.fighterabilitiesbytype)
        string_data.dbuffcollections = json.dumps(self.dbuffcollections)
        string_data.requiredskillsfortypes = json.dumps(self.requiredskillsfortypes)
        string_data.dynamicitemattributes = json.dumps(self.dynamicitemattributes)
        return string_data


class TestStrings:

    def __init__(self, alias):
        self.alias = alias
        self.types = ''
        self.groups = ''
        self.typedogma = ''
        self.dogmaattributes = ''
        self.dogmaeffects = ''
        self.fighterabilities = ''
        self.fighterabilitiesbytype = ''
        self.dbuffcollections = ''
        self.requiredskillsfortypes = ''
        self.dynamicitemattributes = ''
