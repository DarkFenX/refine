class ValMaxGroupDetails(dict):

    def __init__(self, *, data: dict):
        super().__init__({int(k): v for k, v in data.items()})
