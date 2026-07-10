
def is_subset(*, smaller: tuple | list | set | dict, larger: tuple | list | set | dict) -> bool:
    if type(smaller) is not type(larger):
        return False
    # Tuples/lists have to have equal length and proper positions of elements, but their items can
    # be subsets
    if isinstance(smaller, tuple | list) and isinstance(larger, tuple | list):
        return is_subset_sequence(smaller=smaller, larger=larger)
    if isinstance(smaller, set) and isinstance(larger, set):
        return is_subset_set(smaller=smaller, larger=larger)
    if isinstance(smaller, dict) and isinstance(larger, dict):
        return is_subset_dict(smaller=smaller, larger=larger)
    # Primitives
    return smaller == larger


def is_subset_sequence(*, smaller: tuple | list, larger: tuple | list) -> bool:
    if len(smaller) != len(larger):
        return False
    for i, smaller_v in enumerate(smaller):
        larger_v = larger[i]
        if not is_subset(smaller=smaller_v, larger=larger_v):
            return False
    return True


def is_subset_set(*, smaller: set, larger: set) -> bool:
    return smaller.issubset(larger)


def is_subset_dict(*, smaller: dict, larger: dict) -> bool:
    for k, smaller_v in smaller.items():
        try:
            larger_v = larger[k]
        except KeyError:
            return False
        if not is_subset(smaller=smaller_v, larger=larger_v):
            return False
    return True
