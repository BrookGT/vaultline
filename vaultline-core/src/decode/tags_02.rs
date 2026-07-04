//! TLV tag metadata batch 2.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_2: &[TagInfo] = &[
    TagInfo {
        tag: 4166,
        label: "tag-02-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4167,
        label: "tag-02-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4168,
        label: "tag-02-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4169,
        label: "tag-02-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4170,
        label: "tag-02-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4171,
        label: "tag-02-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4172,
        label: "tag-02-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4173,
        label: "tag-02-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4174,
        label: "tag-02-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4175,
        label: "tag-02-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4176,
        label: "tag-02-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4177,
        label: "tag-02-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4178,
        label: "tag-02-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4179,
        label: "tag-02-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4180,
        label: "tag-02-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4181,
        label: "tag-02-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4182,
        label: "tag-02-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4183,
        label: "tag-02-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4184,
        label: "tag-02-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4185,
        label: "tag-02-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4186,
        label: "tag-02-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4187,
        label: "tag-02-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4188,
        label: "tag-02-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4189,
        label: "tag-02-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4190,
        label: "tag-02-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4191,
        label: "tag-02-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4192,
        label: "tag-02-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4193,
        label: "tag-02-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4194,
        label: "tag-02-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4195,
        label: "tag-02-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4196,
        label: "tag-02-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4197,
        label: "tag-02-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4198,
        label: "tag-02-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4199,
        label: "tag-02-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4200,
        label: "tag-02-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4201,
        label: "tag-02-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4202,
        label: "tag-02-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4203,
        label: "tag-02-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4204,
        label: "tag-02-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4205,
        label: "tag-02-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4206,
        label: "tag-02-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4207,
        label: "tag-02-41",
        fixed_len: None,
    },
];

pub fn describe_tag_2(tag: u16) -> Option<&'static TagInfo> {
    TAGS_2.iter().find(|t| t.tag == tag)
}
