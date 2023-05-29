class Buff:

    def __init__(
            self,
            id_,
            aggregate_mode,
            operation_name,
            item_modifiers,
            location_modifiers,
            location_group_modifiers,
            location_skillreq_modifiers,
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
