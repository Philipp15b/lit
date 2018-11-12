use Error;
use std::fmt;
use std::path::PathBuf;

/// A tool invocation.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Invocation
{
    /// The original command string.
    pub original_command: String,
}

// TODO: rename to TestFile
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Test
{
    /// The on-disk path to the test file.
    pub path: PathBuf,
    pub directives: Vec<Directive>,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Directive
{
    pub command: Command,
    pub line: u32,
}

#[derive(Clone,Debug)]
pub enum Command
{
    /// Run an external tool.
    Run(Invocation),
    /// Verify that the output text matches an expression.
    Check(Matcher),
    /// Verify that the very next output line matches an expression.
    CheckNext(Matcher),
    /// Mark the test as supposed to fail.
    XFail,
}

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct Matcher {
    pub components: Vec<Component>,
}

/// A component in a matcher.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Component {
    Text(String),
    Variable(String),
    Regex(String),
    NamedRegex { name: String, regex: String },
}

#[derive(Debug)]
pub enum TestResultKind
{
    Pass,
    UnexpectedPass,
    Error(Error),
    Fail {
        message: String,
        stderr: Option<String>,
    },
    ExpectedFailure,
    Skip,
}

#[derive(Debug)]
pub struct TestResult
{
    pub path: PathBuf,
    pub kind: TestResultKind,
}

impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        match *self {
            Command::Run(ref a) => if let Command::Run(ref b) = *other { a == b } else { false },
            Command::Check(ref a) => if let Command::Check(ref b) = *other { a.to_string() == b.to_string() } else { false },
            Command::CheckNext(ref a) => if let Command::CheckNext(ref b) = *other { a.to_string() == b.to_string() } else { false },
            Command::XFail => *other == Command::XFail,
        }
    }
}

impl Eq for Command { }

impl fmt::Display for Matcher {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for component in self.components.iter() {
            match *component {
                Component::Text(ref text) => write!(fmt, "{}", text)?,
                Component::Variable(ref name) => write!(fmt, "$${}", name)?,
                Component::Regex(ref regex) => write!(fmt, "[[{}]]", regex)?,
                Component::NamedRegex { ref name, ref regex } => write!(fmt, "[[{}:{}]]", name, regex)?,
            }
        }

        Ok(())
    }
}

