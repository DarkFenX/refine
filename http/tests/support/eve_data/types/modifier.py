from tests.support.util import conditional_insert, make_repr_str


class Modifier:

    def __init__(self, func, domain, group, skill_req, src_attr_id, tgt_attr_id, operation):
        self.func = func
        self.domain = domain
        self.group = group
        self.skill_req = skill_req
        self.src_attr_id = src_attr_id
        self.tgt_attr_id = tgt_attr_id
        self.operation = operation

    def to_primitives(self):
        mod_entry = {}
        conditional_insert(mod_entry, 'func', self.func)
        conditional_insert(mod_entry, 'domain', self.domain)
        conditional_insert(mod_entry, 'groupID', self.group)
        conditional_insert(mod_entry, 'skillTypeID', self.skill_req)
        conditional_insert(mod_entry, 'modifyingAttributeID', self.src_attr_id)
        conditional_insert(mod_entry, 'modifiedAttributeID', self.tgt_attr_id)
        conditional_insert(mod_entry, 'operation', self.operation)
        return mod_entry

    def __repr__(self):
        return make_repr_str(self)
