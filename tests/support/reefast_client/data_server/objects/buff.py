from util import Default


class Buff:

    def __init__(
            self,
            buff_id,
            aggregate_mode=Default,
            operation_name=Default,
            item_modifiers=Default,
            location_modifiers=Default,
            location_group_modifiers=Default,
            location_skillreq_modifiers=Default,
    ):
        self.buff_id = buff_id
        self.aggregate_mode = aggregate_mode
        self.operation_name = operation_name
        self.item_modifiers = item_modifiers
        self.location_modifiers = location_modifiers
        self.location_group_modifiers = location_group_modifiers
        self.location_skillreq_modifiers = location_skillreq_modifiers

    def to_primitives(self, primitive_data):
        pass
