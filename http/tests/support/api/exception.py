class ApiRequestError(Exception):

    def __init__(self, expected_code, received_code):
        super().__init__()
        self.expected_code = expected_code
        self.received_code = received_code


class ApiSolCheckError(Exception):
    pass
