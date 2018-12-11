use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename)?;

    let tree: TreeGraph = contents.parse()?;

    Ok(())
}

struct TreeGraph {
    // children
    children: Vec<Box<TreeGraph>>,
    metadata: Vec<i64>,
}

impl FromStr for TreeGraph {
    type Err = Box<error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let mut children_fromstr: Vec<Box<TreeGraph>> = Vec::new();
        let mut metadata_fromstr: Vec<i64> = Vec::new();

        let mut chars = s.split(' ');

        let nchildren = match chars.next() {
            Some(x) => x.parse::<i64>()?,
            None => 0,
        };

        let nmetadata = match chars.next() {
            Some(x) => x.parse::<i64>()?,
            None => 0,
        };

        if nchildren > 0 {
            children_fromstr.append(get_children(s[2..]));
        }

        for _ in 0..nmetadata {
            metadata_fromstr.push(chars.next().unwrap().parse::<i64>()?);
        }

        Ok(TreeGraph {
            children: children_fromstr,
            metadata: metadata_fromstr,
        })
    }
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}