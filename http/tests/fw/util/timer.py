import time


class Timer:

    def __init__(self, *, timeout: int | float = 0):
        self.__timeout = timeout
        self.__start = time.time()

    @property
    def remainder(self) -> float:
        return max(0.0, self.__timeout - self.elapsed)

    @property
    def elapsed(self) -> float:
        return time.time() - self.__start
