//! TLV tag metadata batch 6.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_6: &[TagInfo] = &[
    TagInfo {
        tag: 4306,
        label: "tag-06-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4307,
        label: "tag-06-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4308,
        label: "tag-06-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4309,
        label: "tag-06-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4310,
        label: "tag-06-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4311,
        label: "tag-06-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4312,
        label: "tag-06-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4313,
        label: "tag-06-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4314,
        label: "tag-06-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4315,
        label: "tag-06-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4316,
        label: "tag-06-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4317,
        label: "tag-06-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4318,
        label: "tag-06-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4319,
        label: "tag-06-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4320,
        label: "tag-06-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4321,
        label: "tag-06-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4322,
        label: "tag-06-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4323,
        label: "tag-06-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4324,
        label: "tag-06-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4325,
        label: "tag-06-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4326,
        label: "tag-06-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4327,
        label: "tag-06-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4328,
        label: "tag-06-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4329,
        label: "tag-06-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4330,
        label: "tag-06-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4331,
        label: "tag-06-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4332,
        label: "tag-06-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4333,
        label: "tag-06-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4334,
        label: "tag-06-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4335,
        label: "tag-06-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4336,
        label: "tag-06-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4337,
        label: "tag-06-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4338,
        label: "tag-06-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4339,
        label: "tag-06-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4340,
        label: "tag-06-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4341,
        label: "tag-06-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4342,
        label: "tag-06-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4343,
        label: "tag-06-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4344,
        label: "tag-06-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4345,
        label: "tag-06-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4346,
        label: "tag-06-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4347,
        label: "tag-06-41",
        fixed_len: None,
    },
];

pub fn describe_tag_6(tag: u16) -> Option<&'static TagInfo> {
    TAGS_6.iter().find(|t| t.tag == tag)
}
