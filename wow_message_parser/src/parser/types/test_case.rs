use crate::file_info::FileInfo;
use crate::parser::types::tags::Tags;
use crate::parser::types::{ArraySize, VerifiedContainerValue};
use crate::rust_printer::UpdateMaskType;

#[derive(Debug, Clone)]
pub struct TestCase {
    subject: String,
    members: Vec<TestCaseMember>,
    raw_bytes: Vec<u8>,
    tags: Tags,
    file_info: FileInfo,
}

impl TestCase {
    pub(crate) fn new(
        subject: String,
        members: Vec<TestCaseMember>,
        raw_bytes: Vec<u8>,
        tags: Tags,
        file_info: FileInfo,
    ) -> Self {
        Self {
            subject,
            members,
            raw_bytes,
            tags,
            file_info,
        }
    }

    pub(crate) fn subject(&self) -> &str {
        &self.subject
    }
    pub(crate) fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }

    pub(crate) fn file_info(&self) -> &FileInfo {
        &self.file_info
    }

    pub(crate) fn members(&self) -> &[TestCaseMember] {
        &self.members
    }

    pub(crate) fn get_member<'a>(t: &'a [TestCaseMember], member: &str) -> &'a TestCaseMember {
        t.iter().find(|a| a.name() == member).unwrap_or_else(|| {
            panic!(
                "variable '{}' not found in list of variables with values",
                member
            )
        })
    }
}

#[derive(Debug, Clone)]
pub struct TestCaseMember {
    variable_name: String,
    value: TestValue,
    tags: Tags,
}

impl TestCaseMember {
    pub(crate) fn name(&self) -> &str {
        &self.variable_name
    }

    pub(crate) fn value(&self) -> &TestValue {
        &self.value
    }

    pub(crate) fn tags(&self) -> &Tags {
        &self.tags
    }

    pub(crate) fn new(name: String, value: TestValue, tags: Tags) -> Self {
        Self {
            variable_name: name,
            value,
            tags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TestUpdateMaskValue {
    ty: UpdateMaskType,
    name: String,
    value: String,
}

impl TestUpdateMaskValue {
    pub(crate) fn ty(&self) -> UpdateMaskType {
        self.ty
    }
    pub(crate) fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn new(ty: UpdateMaskType, name: String, value: String) -> Self {
        Self { ty, name, value }
    }
}

#[derive(Debug, Clone)]
pub enum TestValue {
    Number(VerifiedContainerValue),
    Bool(bool),
    DateTime(VerifiedContainerValue),
    Guid(VerifiedContainerValue),
    FloatingNumber {
        value: f64,
        original_string: String,
    },
    Array {
        values: Vec<usize>,
        size: ArraySize,
    },
    String(String),
    Flag(Vec<String>),
    Enum(VerifiedContainerValue),
    SubObject {
        ty_name: String,
        members: Vec<TestCaseMember>,
    },
    ArrayOfSubObject(String, Vec<Vec<TestCaseMember>>),
    UpdateMask(Vec<TestUpdateMaskValue>),
}