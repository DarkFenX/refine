from abc import ABC, abstractmethod


class BaseCommand(ABC):

    @abstractmethod
    def serialize(self) -> dict:
        ...
