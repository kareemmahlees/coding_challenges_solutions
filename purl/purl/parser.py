from abc import ABC, abstractmethod


class BaseParser(ABC):
    @classmethod
    @abstractmethod
    def parse(cls, *args, **kwargs) -> object: ...
