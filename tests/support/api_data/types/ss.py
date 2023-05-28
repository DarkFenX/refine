from tests.support.util import AttrDict
from .fit import Fit
from .item import Item


class SolarSystem(AttrDict):

    def __init__(self, client, data):
        super().__init__(
            data=data,
            hooks={'fits': lambda fits: {f.id: f for f in [Fit(client=client, data=fit, ss_id=self.id) for fit in fits]}})
        object.__setattr__(self, '_client', client)

    def update_request(self):
        return self._client.update_ss_request(ss_id=self.id)

    def update(self):
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json
        return resp

    # Fit-related methods
    def create_fit_request(self):
        return self._client.create_fit_request(ss_id=self.id)

    def create_fit(self):
        resp = self.create_fit_request().send()
        assert resp.status_code == 201
        fit = Fit(client=self._client, data=resp.json(), ss_id=self.id)
        self.update()
        return fit

    # Item-related methods
    def get_item_request(self, item_id):
        return self._client.get_item_request(ss_id=self.id, item_id=item_id)

    def get_item(self, item_id):
        resp = self.get_item_request(item_id=item_id).send()
        assert resp.status_code == 200
        return Item(client=self, data=resp.json(), ss_id=self.id)


