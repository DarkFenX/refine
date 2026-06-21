from abc import ABC, abstractmethod


class Command(ABC):

    @abstractmethod
    def serialize(self) -> dict:
        ...
