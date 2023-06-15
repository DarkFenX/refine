from tests.support.util import conditional_insert, make_repr_str


class Modifier:

    def __init__(self, func, domain, src_attr_id, tgt_attr_id, operation, group):
        self.func = func
        self.domain = domain
        self.src_attr_id = src_attr_id
        self.tgt_attr_id = tgt_attr_id
        self.operation = operation
        self.group = group

    def to_primitives(self):
        mod_entry = {}
        conditional_insert(mod_entry, 'func', self.func)
        conditional_insert(mod_entry, 'domain', self.domain)
        conditional_insert(mod_entry, 'modifyingAttributeID', self.src_attr_id)
        conditional_insert(mod_entry, 'modifiedAttributeID', self.tgt_attr_id)
        conditional_insert(mod_entry, 'operation', self.operation)
        conditional_insert(mod_entry, 'groupID', self.group)
        return mod_entry

    def __repr__(self):
        return make_repr_str(self)
