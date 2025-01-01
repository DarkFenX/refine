from .fit import ApiClientFit
from .fleet import ApiClientFleet
from .item import ApiClientItem
from .sol import ApiClientSol
from .src import ApiClientSrc


class ApiClient(ApiClientFit, ApiClientFleet, ApiClientItem, ApiClientSol, ApiClientSrc):
    pass
