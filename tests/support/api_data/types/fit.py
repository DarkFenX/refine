from tests.support.util import AttrDict
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
    def set_ship_request(self, ship_id):
        return self._client.set_ship_request(ss_id=self._ss_id, fit_id=self.id, ship_id=ship_id)

    def set_ship(self, ship_id):
        resp = self.set_ship_request(ship_id=ship_id).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        item.update()
        return item

    def add_high_mod_request(self, module_id, state='offline', charge_id=None, mode='equip'):
        return self._client.add_high_mod_request(
            ss_id=self._ss_id, fit_id=self.id, module_id=module_id,
            state=state, charge_id=charge_id, mode=mode)

    def add_high_mod(self, module_id, state='offline', charge_id=None, mode='equip'):
        resp = self.add_high_mod_request(module_id=module_id, state=state, charge_id=charge_id, mode=mode).send()
        assert resp.status_code == 200
        item = Item(client=self._client, data=resp.json()['cmd_results'][0], ss_id=self._ss_id)
        return item
