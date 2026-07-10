import json
import typing

from .strings import EveStrings

if typing.TYPE_CHECKING:
    from fw.eve.aliases import DataStrHook


class EvePrimitives:

    def __init__(self, *, alias: str) -> None:
        self.alias = alias
        self.types = {}
        self.groups = {}
        self.typelist = {}
        self.typedogma = {}
        self.dogmaattributes = {}
        self.dogmaeffects = {}
        self.fighterabilities = {}
        self.fighterabilitiesbytype = {}
        self.dbuffcollections = {}
        self.spacecomponentsbytype = {}
        self.requiredskillsfortypes = {}
        self.dynamicitemattributes = {}

    def to_strings(self, *, hook_data_str: DataStrHook | None) -> EveStrings:
        string_data = EveStrings(alias=self.alias)
        string_data.types = json.dumps(self.types)
        string_data.groups = json.dumps(self.groups)
        string_data.typelist = json.dumps(self.typelist)
        string_data.typedogma = json.dumps(self.typedogma)
        string_data.dogmaattributes = json.dumps(self.dogmaattributes)
        string_data.dogmaeffects = json.dumps(self.dogmaeffects)
        string_data.fighterabilities = json.dumps(self.fighterabilities)
        string_data.fighterabilitiesbytype = json.dumps(self.fighterabilitiesbytype)
        string_data.dbuffcollections = json.dumps(self.dbuffcollections)
        string_data.spacecomponentsbytype = json.dumps(self.spacecomponentsbytype)
        string_data.requiredskillsfortypes = json.dumps(self.requiredskillsfortypes)
        string_data.dynamicitemattributes = json.dumps(self.dynamicitemattributes)
        if hook_data_str is not None:
            hook_data_str(string_data)
        return string_data
