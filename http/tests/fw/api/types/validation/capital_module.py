class ValCapModuleDetails(list):

    def __init__(self, *, data: list) -> None:
        super().__init__(sorted(data))
