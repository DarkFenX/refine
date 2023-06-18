from typing import Any

from tests.support.util import make_repr_str


class Buff:

    def __init__(
            self,
            id_: Any,
            aggregate_mode: Any,
            operation_name: Any,
            item_modifiers: Any,
            location_modifiers: Any,
            location_group_modifiers: Any,
            location_skillreq_modifiers: Any,
    ):
        self.id = id_
        self.aggregate_mode = aggregate_mode
        self.operation_name = operation_name
        self.item_modifiers = item_modifiers
        self.location_modifiers = location_modifiers
        self.location_group_modifiers = location_group_modifiers
        self.location_skillreq_modifiers = location_skillreq_modifiers

    def to_primitives(self, primitive_data):
        pass

    def __repr__(self) -> str:
        return make_repr_str(self)
