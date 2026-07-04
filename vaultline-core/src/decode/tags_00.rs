//! TLV tag metadata batch 0.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_0: &[TagInfo] = &[
    TagInfo {
        tag: 4096,
        label: "tag-00-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4097,
        label: "tag-00-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4098,
        label: "tag-00-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4099,
        label: "tag-00-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4100,
        label: "tag-00-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4101,
        label: "tag-00-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4102,
        label: "tag-00-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4103,
        label: "tag-00-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4104,
        label: "tag-00-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4105,
        label: "tag-00-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4106,
        label: "tag-00-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4107,
        label: "tag-00-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4108,
        label: "tag-00-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4109,
        label: "tag-00-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4110,
        label: "tag-00-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4111,
        label: "tag-00-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4112,
        label: "tag-00-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4113,
        label: "tag-00-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4114,
        label: "tag-00-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4115,
        label: "tag-00-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4116,
        label: "tag-00-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4117,
        label: "tag-00-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4118,
        label: "tag-00-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4119,
        label: "tag-00-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4120,
        label: "tag-00-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4121,
        label: "tag-00-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4122,
        label: "tag-00-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4123,
        label: "tag-00-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4124,
        label: "tag-00-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4125,
        label: "tag-00-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4126,
        label: "tag-00-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4127,
        label: "tag-00-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4128,
        label: "tag-00-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4129,
        label: "tag-00-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4130,
        label: "tag-00-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4131,
        label: "tag-00-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4132,
        label: "tag-00-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4133,
        label: "tag-00-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4134,
        label: "tag-00-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4135,
        label: "tag-00-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4136,
        label: "tag-00-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4137,
        label: "tag-00-41",
        fixed_len: None,
    },
];

pub fn describe_tag_0(tag: u16) -> Option<&'static TagInfo> {
    TAGS_0.iter().find(|t| t.tag == tag)
}
