use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum TemplateType {
    Default = 0,
    Remote = 1,
}

impl PartialEq for TemplateType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Template {
    pub name: String,
    pub template_type: TemplateType,
}

impl Template {
    pub fn new(name: &str, template_type: TemplateType) -> Self {
        Self {
            name: name.to_owned(),
            template_type,
        }
    }
}

impl PartialEq for Template {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.template_type == other.template_type
    }
}

impl PartialOrd for Template {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        (self.template_type as i32).partial_cmp(&(other.template_type as i32))
    }
}

impl Eq for Template {}

impl Ord for Template {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[macro_export]
macro_rules! template {
    ($a:expr,$b:expr) => {
        Template::new($a, $b)
    };
    ($a:expr) => {
        Template::new($a, TemplateType::Default)
    };
}
