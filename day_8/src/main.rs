use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename)?;
    let mut split = contents.split(" ");

    let tree: TreeGraph = parse_node(&mut split)?;

    let total_metadata: usize = metadata_total(&tree);

    println!("{}", total_metadata);

    Ok(())
}

#[derive(Debug)]
struct TreeGraph {
    // children
    children: Vec<Box<TreeGraph>>,
    metadata: Vec<usize>,
}

fn metadata_total(tree: &TreeGraph) -> usize {
    let mut res: usize = 0;

    for m in (*tree).metadata.iter() {
        res += m;
    }

    for child in (*tree).children.iter() {
        res += metadata_total(&**child);
    }

    res
}

fn parse_node(s: &mut Split<&str>) -> Result<TreeGraph> {
    let mut children_fromstr: Vec<Box<TreeGraph>> = Vec::new();
    let mut metadata_fromstr: Vec<usize> = Vec::new();

    let nchildren = match s.next() {
        Some(x) => x.to_string().parse::<usize>()?,
        None => 0,
    };

    let nmetadata = match s.next() {
        Some(x) => x.to_string().parse::<usize>()?,
        None => 0,
    };

    for _ in 0..nchildren {
        let g: TreeGraph = parse_node(s)?;
        children_fromstr.push(Box::new(g));
    }

    for _ in 0..nmetadata {
        metadata_fromstr.push(s.next().unwrap().to_string().parse::<usize>()?);
    }

    Ok(TreeGraph {
        children: children_fromstr,
        metadata: metadata_fromstr,
    })
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}
