import json

from .objects import Item, Attribute, Effect, Buff
from ...util import Default


ID_START = 1000000


class TestObjects:

    def __init__(self):
        self.items = []
        self.attributes = []
        self.effects = []
        self.buffs = []
        self.item_id = ID_START
        self.item_group_id = ID_START
        self.item_category_id = ID_START
        self.attr_id = ID_START
        self.effect_id = ID_START
        self.buff_id = ID_START

    def add_item(self, id, group_id, category_id, *args, **kwargs):
        if id is Default:
            id = self.item_id
            self.item_id += 1
        if group_id is Default:
            group_id = self.item_group_id
            self.item_group_id += 1
        if category_id is Default:
            category_id = self.item_category_id
            self.item_category_id += 1
        item = Item(id=id, group_id=group_id, category_id=category_id, *args, **kwargs)
        self.items.append(item)
        return item

    def add_attr(self, id, *args, **kwargs):
        if id is Default:
            id = self.attr_id
            self.attr_id += 1
        attr = Attribute(id, *args, **kwargs)
        self.attributes.append(attr)
        return attr

    def add_effect(self, id, *args, **kwargs):
        if id is Default:
            id = self.effect_id
            self.effect_id += 1
        effect = Effect(id, *args, **kwargs)
        self.effects.append(effect)
        return effect

    def add_buff(self, id, *args, **kwargs):
        if id is Default:
            id = self.buff_id
            self.buff_id += 1
        buff = Buff(id, *args, **kwargs)
        self.buffs.append(buff)
        return buff

    def render(self):
        primitives = self.to_primitives()
        strings = primitives.to_strings()
        return strings

    def to_primitives(self):
        primitive_data = TestPrimitives()
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

    def __init__(self):
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
        string_data = TestStrings()
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


class TestStrings:

    def __init__(self):
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
        
