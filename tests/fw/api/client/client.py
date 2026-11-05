from .fit import ApiClientFit
from .fleet import ApiClientFleet
from .item import ApiClientItem
from .refine import ApiClientRefine
from .sol import ApiClientSol
from .src import ApiClientSrc


class ApiClient(ApiClientFit, ApiClientFleet, ApiClientItem, ApiClientRefine, ApiClientSol, ApiClientSrc):
    ...
