

def test_penalized(client):
    src_attr = client.add_attr()
    tgt_attr = client.add_attr(stackable=False)
    effect = client.add_effect()
    item = client.add_item(attrs={src_attr.id: 5.2, tgt_attr.id: 3.1}, eff_ids=[effect.id])


def test_non_penalized(client, consts):
    src_attr = client.add_attr()
    tgt_attr = client.add_attr(stackable=False)
    effect = client.add_effect()
    item = client.add_item(
        cat_id=consts.ItemCategory.ship,
        attrs={src_attr.id: 5.2, tgt_attr.id: 3.1},
        eff_ids=[effect.id])
