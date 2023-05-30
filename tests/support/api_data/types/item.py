from collections import namedtuple

from tests.support.util import AttrDict

AttrVals = namedtuple('AttrVals', ('base', 'dogma', 'extra'))


class Item(AttrDict):

    def __init__(self, client, data, ss_id):
        super().__init__(
            data=data,
            hooks={'attr_vals': lambda attr_vals: {int(k): AttrVals(*v) for k, v in attr_vals.items()}})
        self._client = client
        self._ss_id = ss_id

    def update_request(self):
        return self._client.get_item_request(ss_id=self._ss_id, item_id=self.id)

    def update(self):
        resp = self.update_request().send()
        assert resp.status_code == 200
        self._data = resp.json()
        return self

    def remove_request(self):
        return self._client.remove_item_request(ss_id=self._ss_id, item_id=self.id)

    def remove(self):
        resp = self.remove_request().send()
        assert resp.status_code == 204
