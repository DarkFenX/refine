

def test_penalized(client):
    src_attr = client.mk_attr()
    tgt_attr = client.mk_attr(stackable=False)
    effect = client.mk_effect()
    item = client.mk_item(attrs={src_attr.id: 5.2, tgt_attr.id: 3.1}, eff_ids=[effect.id])
    client.create_sources()


def test_non_penalized(client, consts):
    src_attr = client.mk_attr()
    tgt_attr = client.mk_attr(stackable=False)
    effect = client.mk_effect()
    item = client.mk_item(
        cat_id=consts.ItemCategory.ship,
        attrs={src_attr.id: 5.2, tgt_attr.id: 3.1},
        eff_ids=[effect.id])
    client.create_sources()
