

def make_repr_str(instance, spec=None):
    arg_list = []
    if spec is None:
        for attr_name in sorted(vars(instance)):
            attr_val = getattr(instance, attr_name)
            arg_list.append(f'{attr_name}={attr_val}')
    else:
        for field in spec:
            if isinstance(field, str):
                repr_name, attr_name = field, field
            else:
                repr_name, attr_name = field
            attr_val = getattr(instance, attr_name, 'N/A')
            arg_list.append(f'{repr_name}={attr_val}')
    args = ', '.join(arg_list)
    return f'{type(instance).__name__}({args})'
