//! TLV tag metadata batch 7.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_7: &[TagInfo] = &[
    TagInfo {
        tag: 4341,
        label: "tag-07-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4342,
        label: "tag-07-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4343,
        label: "tag-07-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4344,
        label: "tag-07-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4345,
        label: "tag-07-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4346,
        label: "tag-07-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4347,
        label: "tag-07-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4348,
        label: "tag-07-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4349,
        label: "tag-07-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4350,
        label: "tag-07-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4351,
        label: "tag-07-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4352,
        label: "tag-07-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4353,
        label: "tag-07-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4354,
        label: "tag-07-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4355,
        label: "tag-07-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4356,
        label: "tag-07-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4357,
        label: "tag-07-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4358,
        label: "tag-07-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4359,
        label: "tag-07-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4360,
        label: "tag-07-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4361,
        label: "tag-07-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4362,
        label: "tag-07-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4363,
        label: "tag-07-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4364,
        label: "tag-07-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4365,
        label: "tag-07-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4366,
        label: "tag-07-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4367,
        label: "tag-07-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4368,
        label: "tag-07-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4369,
        label: "tag-07-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4370,
        label: "tag-07-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4371,
        label: "tag-07-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4372,
        label: "tag-07-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4373,
        label: "tag-07-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4374,
        label: "tag-07-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4375,
        label: "tag-07-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4376,
        label: "tag-07-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4377,
        label: "tag-07-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4378,
        label: "tag-07-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4379,
        label: "tag-07-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4380,
        label: "tag-07-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4381,
        label: "tag-07-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4382,
        label: "tag-07-41",
        fixed_len: None,
    },
];

pub fn describe_tag_7(tag: u16) -> Option<&'static TagInfo> {
    TAGS_7.iter().find(|t| t.tag == tag)
}
