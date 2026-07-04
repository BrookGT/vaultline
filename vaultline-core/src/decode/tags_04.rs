//! TLV tag metadata batch 4.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_4: &[TagInfo] = &[
    TagInfo {
        tag: 4236,
        label: "tag-04-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4237,
        label: "tag-04-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4238,
        label: "tag-04-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4239,
        label: "tag-04-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4240,
        label: "tag-04-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4241,
        label: "tag-04-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4242,
        label: "tag-04-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4243,
        label: "tag-04-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4244,
        label: "tag-04-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4245,
        label: "tag-04-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4246,
        label: "tag-04-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4247,
        label: "tag-04-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4248,
        label: "tag-04-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4249,
        label: "tag-04-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4250,
        label: "tag-04-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4251,
        label: "tag-04-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4252,
        label: "tag-04-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4253,
        label: "tag-04-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4254,
        label: "tag-04-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4255,
        label: "tag-04-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4256,
        label: "tag-04-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4257,
        label: "tag-04-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4258,
        label: "tag-04-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4259,
        label: "tag-04-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4260,
        label: "tag-04-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4261,
        label: "tag-04-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4262,
        label: "tag-04-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4263,
        label: "tag-04-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4264,
        label: "tag-04-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4265,
        label: "tag-04-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4266,
        label: "tag-04-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4267,
        label: "tag-04-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4268,
        label: "tag-04-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4269,
        label: "tag-04-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4270,
        label: "tag-04-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4271,
        label: "tag-04-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4272,
        label: "tag-04-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4273,
        label: "tag-04-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4274,
        label: "tag-04-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4275,
        label: "tag-04-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4276,
        label: "tag-04-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4277,
        label: "tag-04-41",
        fixed_len: None,
    },
];

pub fn describe_tag_4(tag: u16) -> Option<&'static TagInfo> {
    TAGS_4.iter().find(|t| t.tag == tag)
}
