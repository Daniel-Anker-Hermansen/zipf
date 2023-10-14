use std::borrow::Cow;

#[derive(Debug, Clone)]
enum PathComponent {
    Wildcard,
    Str(String),
}

impl PathComponent {
    fn from_string(str: &str) -> PathComponent {
        match str {
            "*" => PathComponent::Wildcard,
            _ => PathComponent::Str(str.to_owned())
        }
    }

    fn matches(&self, f: &str) -> bool {
        match self {
            PathComponent::Wildcard => true,
            PathComponent::Str(s) => f == s,
        }
    }
}

#[derive(Debug, Clone)]
enum Component {
    Wildcard,
    Path(Vec<PathComponent>),
}

impl Component {
    fn from_string(str: &str) -> Component {
        match str {
            "*" | "." | "" => Component::Wildcard,
            _ => {
                let components = str.split(".").map(PathComponent::from_string).collect();
                Component::Path(components)
            },
        }
    }

    fn matches(&self, file: &str) -> bool {
        match self {
            Component::Wildcard => true,
            Component::Path(path_components) => {
                let compnent_cnt = file.matches(".").count() + 1;
                if compnent_cnt != path_components.len() {
                    false
                }
                else {
                    file.split(".").zip(path_components).all(|(f, c)| c.matches(f))
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item<'a> {
    path: Cow<'a, [Component]>,
}

pub enum Matches<'a> {
    Exact,
    Partial(Item<'a>),
    No,
}

const WILDCARD: &[Component] = &[Component::Wildcard];

impl Item<'_> {
    pub fn from_string(str: &str) -> Item<'static> {
        let components = str.split("/").map(Component::from_string).collect();
        Item { path: Cow::Owned(components) }
    }

    fn slice(&self) -> Item<'_> {
        let components = &self.path[1..];
        Item { path: Cow::Borrowed(components) }
    }
    
    fn wildcard() -> Item<'static> {
        Item { path: Cow::Borrowed(WILDCARD) }
    }

    pub fn recurse(&self, file: &str) -> Matches<'_> {
        let head = &self.path[0];
        let file = file.split("/").last().expect("split always has a last");
        if head.matches(file) {
            if self.path.len() == 1 {
                Matches::Exact
            }
            else {
                Matches::Partial(self.slice())
            }
        }
        else {
            Matches::No
        }
    }
}

#[derive(Debug)]
pub enum CollectionMatches<'a> {
    Exact(Vec<Item<'a>>),
    Partial(Vec<Item<'a>>),
    No,
}

pub fn recurse_collection<'a>(file: &str, items: &'a [Item<'a>]) -> CollectionMatches<'a> {
    let mut exact = false;
    let mut acc = Vec::new();
    for item in items {
        match item.recurse(file) {
            Matches::Exact => {
                exact = true;
                acc.push(Item::wildcard());
            },
            Matches::Partial(p) => acc.push(p),
            Matches::No => (),
        }
    }
    if acc.is_empty() {
        CollectionMatches::No
    }
    else {
        if exact {
            CollectionMatches::Exact(acc)
        }
        else {
            CollectionMatches::Partial(acc)
        }
    }
}
