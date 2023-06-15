from tests.support.util import AttrDict, Absent
from .item import Item


class Fit(AttrDict):

    def __init__(self, client, data, ss_id):
        super().__init__(data=data)
        self._client = client
        self._ss_id = ss_id

    def update_request(self):
        return self._client.update_fit_request(ss_id=self._ss_id, fit_id=self.id)

    def update(self):
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    # Item-related methods
    def remove_item_request(self, item_id):
        return self._client.remove_item_request(ss_id=self._ss_id, item_id=item_id)

    def remove_item(self, item_id):
        resp = self.remove_item_request(item_id=item_id).send()
        assert resp.status_code == 204

    def set_char_request(self, type_id):
        return self._client.set_char_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id)

    def set_char(self, type_id):
        resp = self.set_char_request(type_id=type_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    def add_implant_request(self, type_id, state=Absent):
        return self._client.add_implant_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_implant(self, type_id, state=Absent):
        resp = self.add_implant_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    def set_ship_request(self, type_id):
        return self._client.set_ship_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id)

    def set_ship(self, type_id):
        resp = self.set_ship_request(type_id=type_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    def add_high_mod_request(self, module_type_id, state='offline', charge_type_id=None, mode='equip'):
        return self._client.add_high_mod_request(
            ss_id=self._ss_id, fit_id=self.id, module_type_id=module_type_id,
            state=state, charge_type_id=charge_type_id, mode=mode)

    def add_high_mod(self, module_type_id, state='offline', charge_type_id=None, mode='equip'):
        resp = self.add_high_mod_request(
            module_type_id=module_type_id,
            state=state,
            charge_type_id=charge_type_id,
            mode=mode).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item

    def add_rig_request(self, type_id, state=Absent):
        return self._client.add_rig_request(ss_id=self._ss_id, fit_id=self.id, type_id=type_id, state=state)

    def add_rig(self, type_id, state=Absent):
        resp = self.add_rig_request(type_id=type_id, state=state).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item
