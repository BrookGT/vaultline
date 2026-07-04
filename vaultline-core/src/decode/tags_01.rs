//! TLV tag metadata batch 1.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_1: &[TagInfo] = &[
    TagInfo {
        tag: 4131,
        label: "tag-01-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4132,
        label: "tag-01-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4133,
        label: "tag-01-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4134,
        label: "tag-01-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4135,
        label: "tag-01-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4136,
        label: "tag-01-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4137,
        label: "tag-01-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4138,
        label: "tag-01-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4139,
        label: "tag-01-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4140,
        label: "tag-01-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4141,
        label: "tag-01-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4142,
        label: "tag-01-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4143,
        label: "tag-01-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4144,
        label: "tag-01-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4145,
        label: "tag-01-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4146,
        label: "tag-01-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4147,
        label: "tag-01-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4148,
        label: "tag-01-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4149,
        label: "tag-01-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4150,
        label: "tag-01-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4151,
        label: "tag-01-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4152,
        label: "tag-01-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4153,
        label: "tag-01-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4154,
        label: "tag-01-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4155,
        label: "tag-01-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4156,
        label: "tag-01-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4157,
        label: "tag-01-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4158,
        label: "tag-01-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4159,
        label: "tag-01-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4160,
        label: "tag-01-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4161,
        label: "tag-01-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4162,
        label: "tag-01-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4163,
        label: "tag-01-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4164,
        label: "tag-01-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4165,
        label: "tag-01-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4166,
        label: "tag-01-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4167,
        label: "tag-01-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4168,
        label: "tag-01-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4169,
        label: "tag-01-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4170,
        label: "tag-01-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4171,
        label: "tag-01-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4172,
        label: "tag-01-41",
        fixed_len: None,
    },
];

pub fn describe_tag_1(tag: u16) -> Option<&'static TagInfo> {
    TAGS_1.iter().find(|t| t.tag == tag)
}
