//! TLV tag metadata batch 3.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_3: &[TagInfo] = &[
    TagInfo {
        tag: 4201,
        label: "tag-03-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4202,
        label: "tag-03-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4203,
        label: "tag-03-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4204,
        label: "tag-03-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4205,
        label: "tag-03-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4206,
        label: "tag-03-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4207,
        label: "tag-03-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4208,
        label: "tag-03-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4209,
        label: "tag-03-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4210,
        label: "tag-03-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4211,
        label: "tag-03-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4212,
        label: "tag-03-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4213,
        label: "tag-03-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4214,
        label: "tag-03-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4215,
        label: "tag-03-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4216,
        label: "tag-03-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4217,
        label: "tag-03-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4218,
        label: "tag-03-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4219,
        label: "tag-03-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4220,
        label: "tag-03-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4221,
        label: "tag-03-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4222,
        label: "tag-03-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4223,
        label: "tag-03-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4224,
        label: "tag-03-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4225,
        label: "tag-03-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4226,
        label: "tag-03-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4227,
        label: "tag-03-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4228,
        label: "tag-03-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4229,
        label: "tag-03-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4230,
        label: "tag-03-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4231,
        label: "tag-03-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4232,
        label: "tag-03-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4233,
        label: "tag-03-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4234,
        label: "tag-03-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4235,
        label: "tag-03-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4236,
        label: "tag-03-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4237,
        label: "tag-03-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4238,
        label: "tag-03-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4239,
        label: "tag-03-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4240,
        label: "tag-03-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4241,
        label: "tag-03-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4242,
        label: "tag-03-41",
        fixed_len: None,
    },
];

pub fn describe_tag_3(tag: u16) -> Option<&'static TagInfo> {
    TAGS_3.iter().find(|t| t.tag == tag)
}
