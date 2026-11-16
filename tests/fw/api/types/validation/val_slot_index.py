from collections import UserDict

from fw.consts import ApiValItemStatus


class ValSlotIndexFail(UserDict):

    def __init__(self, *, data: dict) -> None:
        super().__init__({
            int(slot): {item_id: ApiValItemStatus(status) for item_id, status in items.items()}
            for slot, items in data.items()})
