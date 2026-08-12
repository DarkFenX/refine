use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct BuffIm {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
}
impl BuffIm {
    pub(crate) fn into_e_buff_mod(self) -> rc::ed::EBuffIm {
        rc::ed::EBuffIm {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct BuffLm {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
}
impl BuffLm {
    pub(crate) fn into_e_buff_mod(self) -> rc::ed::EBuffLm {
        rc::ed::EBuffLm {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct BuffLgm {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
    #[serde(rename = "groupID")]
    group_id: i32,
}
impl BuffLgm {
    pub(crate) fn into_e_buff_mod(self) -> rc::ed::EBuffLgm {
        rc::ed::EBuffLgm {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            group_id: rc::ed::EItemGrpId::from_i32(self.group_id),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct BuffLrsm {
    #[serde(rename = "dogmaAttributeID")]
    attr_id: i32,
    #[serde(rename = "skillID")]
    skill_id: i32,
}
impl BuffLrsm {
    pub(crate) fn into_e_buff_mod(self) -> rc::ed::EBuffLrsm {
        rc::ed::EBuffLrsm {
            attr_id: rc::ed::EAttrId::from_i32(self.attr_id),
            skill_id: rc::ed::EItemId::from_i32(self.skill_id),
        }
    }
}
