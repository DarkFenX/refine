import typing

from fw.util import Absent


def process_stats_options_request(*, options: typing.Any) -> typing.Any:
    if options is Absent:
        return options
    if isinstance(options, tuple | list):
        default, overrides = options
        return [default.to_dict(), [[o.to_dict(), list(ids)] for o, ids in overrides]]
    return options.to_dict()


def process_val_options_request(*, options: typing.Any) -> typing.Any:
    if options is Absent:
        return options
    return options.to_dict()
