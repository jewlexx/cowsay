use include_dir::{include_dir, Dir};

static COWS: Dir = include_dir!("cows");

pub struct Cow {
    pub name: String,
    pub template: String,
}

impl Cow {
    pub fn load(name: impl AsRef<str>) -> Option<Self> {
        let path = name.as_ref().to_string() + ".cow";
        let file = COWS.get_file(&path)?;
        let contents = file.contents_utf8()?;

        // TODO: Add cowpath

        Some(Self {
            name: path,
            template: contents.to_string(),
        })
    }

    pub fn render(&self) -> String {
        self.template.replace("{{name}}", &self.name)
    }
}
