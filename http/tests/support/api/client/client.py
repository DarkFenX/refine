from .base import ApiClientBase
from .sol import ApiClientSol
from .src import ApiClientSrc


class ApiClient(ApiClientSol, ApiClientSrc, ApiClientBase):
    pass
