class ApiRequestError(Exception):

    def __init__(self, expected_code: int, received_code: int):
        super().__init__(f'expected {expected_code}, received {received_code}')
        self.expected_code = expected_code
        self.received_code = received_code

    @property
    def message(self):
        return


class ApiSolCheckError(Exception):
    pass
