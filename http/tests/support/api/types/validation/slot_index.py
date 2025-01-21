class ValSlotIndexDetails(dict):

    def __init__(self, *, data: dict):
        super().__init__({int(k): sorted(v) for k, v in data.items()})
