//! TLV tag metadata batch 9.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_9: &[TagInfo] = &[
    TagInfo {
        tag: 4411,
        label: "tag-09-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4412,
        label: "tag-09-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4413,
        label: "tag-09-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4414,
        label: "tag-09-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4415,
        label: "tag-09-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4416,
        label: "tag-09-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4417,
        label: "tag-09-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4418,
        label: "tag-09-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4419,
        label: "tag-09-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4420,
        label: "tag-09-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4421,
        label: "tag-09-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4422,
        label: "tag-09-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4423,
        label: "tag-09-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4424,
        label: "tag-09-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4425,
        label: "tag-09-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4426,
        label: "tag-09-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4427,
        label: "tag-09-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4428,
        label: "tag-09-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4429,
        label: "tag-09-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4430,
        label: "tag-09-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4431,
        label: "tag-09-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4432,
        label: "tag-09-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4433,
        label: "tag-09-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4434,
        label: "tag-09-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4435,
        label: "tag-09-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4436,
        label: "tag-09-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4437,
        label: "tag-09-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4438,
        label: "tag-09-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4439,
        label: "tag-09-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4440,
        label: "tag-09-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4441,
        label: "tag-09-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4442,
        label: "tag-09-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4443,
        label: "tag-09-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4444,
        label: "tag-09-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4445,
        label: "tag-09-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4446,
        label: "tag-09-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4447,
        label: "tag-09-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4448,
        label: "tag-09-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4449,
        label: "tag-09-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4450,
        label: "tag-09-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4451,
        label: "tag-09-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4452,
        label: "tag-09-41",
        fixed_len: None,
    },
];

pub fn describe_tag_9(tag: u16) -> Option<&'static TagInfo> {
    TAGS_9.iter().find(|t| t.tag == tag)
}
