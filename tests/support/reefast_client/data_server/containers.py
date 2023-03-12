import json

class TestData:

    def __init__(self):
        self.items = []
        self.attrs = []
        self.effects = []
        self.buffs = []

    def add_item(self, item):
        self.items.append(item)

    def add_attr(self, attr):
        self.attrs.append(attr)

    def add_effect(self, effect):
        self.effects.append(effect)

    def add_buff(self, buff):
        self.buffs.append(buff)

    def render(self):
        primitives = self.to_primitives()

    def to_primitives(self):
        primitive_data = TestPrimitives()
        for item in self.items:
            item.to_primitives(primitive_data)
        for attr in self.attrs:
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
        
