//! TLV tag metadata batch 8.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_8: &[TagInfo] = &[
    TagInfo {
        tag: 4376,
        label: "tag-08-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4377,
        label: "tag-08-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4378,
        label: "tag-08-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4379,
        label: "tag-08-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4380,
        label: "tag-08-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4381,
        label: "tag-08-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4382,
        label: "tag-08-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4383,
        label: "tag-08-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4384,
        label: "tag-08-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4385,
        label: "tag-08-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4386,
        label: "tag-08-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4387,
        label: "tag-08-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4388,
        label: "tag-08-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4389,
        label: "tag-08-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4390,
        label: "tag-08-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4391,
        label: "tag-08-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4392,
        label: "tag-08-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4393,
        label: "tag-08-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4394,
        label: "tag-08-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4395,
        label: "tag-08-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4396,
        label: "tag-08-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4397,
        label: "tag-08-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4398,
        label: "tag-08-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4399,
        label: "tag-08-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4400,
        label: "tag-08-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4401,
        label: "tag-08-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4402,
        label: "tag-08-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4403,
        label: "tag-08-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4404,
        label: "tag-08-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4405,
        label: "tag-08-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4406,
        label: "tag-08-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4407,
        label: "tag-08-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4408,
        label: "tag-08-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4409,
        label: "tag-08-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4410,
        label: "tag-08-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4411,
        label: "tag-08-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4412,
        label: "tag-08-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4413,
        label: "tag-08-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4414,
        label: "tag-08-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4415,
        label: "tag-08-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4416,
        label: "tag-08-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4417,
        label: "tag-08-41",
        fixed_len: None,
    },
];

pub fn describe_tag_8(tag: u16) -> Option<&'static TagInfo> {
    TAGS_8.iter().find(|t| t.tag == tag)
}
