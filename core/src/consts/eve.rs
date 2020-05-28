pub(crate) mod attrs {
    use crate::defines::ReeInt;

    pub(crate) const WARFARE_BUFF1_ID: ReeInt = 2468;
    pub(crate) const WARFARE_BUFF2_ID: ReeInt = 2470;
    pub(crate) const WARFARE_BUFF3_ID: ReeInt = 2472;
    pub(crate) const WARFARE_BUFF4_ID: ReeInt = 2536;

    pub(crate) const BUFF_ID_ATTRS: [ReeInt; 4] =
        [WARFARE_BUFF1_ID, WARFARE_BUFF2_ID, WARFARE_BUFF3_ID, WARFARE_BUFF4_ID];
}

pub(crate) mod units {
    use crate::defines::ReeInt;

    pub(crate) const GROUP_ID: ReeInt = 115;
    pub(crate) const ITEM_ID: ReeInt = 116;
    pub(crate) const ATTR_ID: ReeInt = 119;
}
