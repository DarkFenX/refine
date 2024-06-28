class ApiRequestError(Exception):

    def __init__(self, expected_code: int, received_code: int):
        super().__init__()
        self.expected_code = expected_code
        self.received_code = received_code


class ApiSolCheckError(Exception):
    pass
