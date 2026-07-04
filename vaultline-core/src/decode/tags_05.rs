//! TLV tag metadata batch 5.

#[derive(Debug, Clone, Copy)]
pub struct TagInfo {
    pub tag: u16,
    pub label: &'static str,
    pub fixed_len: Option<usize>,
}

pub const TAGS_5: &[TagInfo] = &[
    TagInfo {
        tag: 4271,
        label: "tag-05-00",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4272,
        label: "tag-05-01",
        fixed_len: None,
    },
    TagInfo {
        tag: 4273,
        label: "tag-05-02",
        fixed_len: None,
    },
    TagInfo {
        tag: 4274,
        label: "tag-05-03",
        fixed_len: None,
    },
    TagInfo {
        tag: 4275,
        label: "tag-05-04",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4276,
        label: "tag-05-05",
        fixed_len: None,
    },
    TagInfo {
        tag: 4277,
        label: "tag-05-06",
        fixed_len: None,
    },
    TagInfo {
        tag: 4278,
        label: "tag-05-07",
        fixed_len: None,
    },
    TagInfo {
        tag: 4279,
        label: "tag-05-08",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4280,
        label: "tag-05-09",
        fixed_len: None,
    },
    TagInfo {
        tag: 4281,
        label: "tag-05-10",
        fixed_len: None,
    },
    TagInfo {
        tag: 4282,
        label: "tag-05-11",
        fixed_len: None,
    },
    TagInfo {
        tag: 4283,
        label: "tag-05-12",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4284,
        label: "tag-05-13",
        fixed_len: None,
    },
    TagInfo {
        tag: 4285,
        label: "tag-05-14",
        fixed_len: None,
    },
    TagInfo {
        tag: 4286,
        label: "tag-05-15",
        fixed_len: None,
    },
    TagInfo {
        tag: 4287,
        label: "tag-05-16",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4288,
        label: "tag-05-17",
        fixed_len: None,
    },
    TagInfo {
        tag: 4289,
        label: "tag-05-18",
        fixed_len: None,
    },
    TagInfo {
        tag: 4290,
        label: "tag-05-19",
        fixed_len: None,
    },
    TagInfo {
        tag: 4291,
        label: "tag-05-20",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4292,
        label: "tag-05-21",
        fixed_len: None,
    },
    TagInfo {
        tag: 4293,
        label: "tag-05-22",
        fixed_len: None,
    },
    TagInfo {
        tag: 4294,
        label: "tag-05-23",
        fixed_len: None,
    },
    TagInfo {
        tag: 4295,
        label: "tag-05-24",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4296,
        label: "tag-05-25",
        fixed_len: None,
    },
    TagInfo {
        tag: 4297,
        label: "tag-05-26",
        fixed_len: None,
    },
    TagInfo {
        tag: 4298,
        label: "tag-05-27",
        fixed_len: None,
    },
    TagInfo {
        tag: 4299,
        label: "tag-05-28",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4300,
        label: "tag-05-29",
        fixed_len: None,
    },
    TagInfo {
        tag: 4301,
        label: "tag-05-30",
        fixed_len: None,
    },
    TagInfo {
        tag: 4302,
        label: "tag-05-31",
        fixed_len: None,
    },
    TagInfo {
        tag: 4303,
        label: "tag-05-32",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4304,
        label: "tag-05-33",
        fixed_len: None,
    },
    TagInfo {
        tag: 4305,
        label: "tag-05-34",
        fixed_len: None,
    },
    TagInfo {
        tag: 4306,
        label: "tag-05-35",
        fixed_len: None,
    },
    TagInfo {
        tag: 4307,
        label: "tag-05-36",
        fixed_len: Some(8),
    },
    TagInfo {
        tag: 4308,
        label: "tag-05-37",
        fixed_len: None,
    },
    TagInfo {
        tag: 4309,
        label: "tag-05-38",
        fixed_len: None,
    },
    TagInfo {
        tag: 4310,
        label: "tag-05-39",
        fixed_len: None,
    },
    TagInfo {
        tag: 4311,
        label: "tag-05-40",
        fixed_len: Some(4),
    },
    TagInfo {
        tag: 4312,
        label: "tag-05-41",
        fixed_len: None,
    },
];

pub fn describe_tag_5(tag: u16) -> Option<&'static TagInfo> {
    TAGS_5.iter().find(|t| t.tag == tag)
}
