use crate::AsGroupName;

#[derive(Debug)]
pub struct Link<'a> {
    to_group: &'a str,
    force: bool,
    default: bool,
}

impl<'a> Link<'a> {
    pub fn set<T: AsGroupName>(to_group: T) -> Self {
        Self {
            to_group: to_group.as_group_name(),
            force: false,
            default: false,
        }
    }

    pub fn remove() -> Self {
        Self {
            to_group: "NONE",
            force: false,
            default: false,
        }
    }

    pub fn set_force(&mut self, force: bool) -> &mut Self {
        self.force = force;
        self
    }

    pub fn set_default(&mut self, default: bool) -> &mut Self {
        self.default = default;
        self
    }

    pub fn cmd<U: AsGroupName>(&self, from_group: U) -> String {
        let mut output = String::from("highlight");

        if self.force {
            output.push('!');
        }

        let mut chunks = vec![output.as_str()];

        if self.default {
            chunks.push("default");
        }

        chunks.extend_from_slice(&["link", from_group.as_group_name(), self.to_group]);

        chunks.join(" ")
    }
}
