// https://git-scm.com/book/en/v2/Git-Internals-Git-Objects
use git2::{Repository, Tree, Commit};
use std::fs;
use std::path::Path;
use std::str::{FromStr, from_utf8};
use serde::Deserialize;
use git2::DiffFormat::NameOnly;

#[derive(Deserialize)]
struct File {
    name: String,
    content: String,
}

// TODO: Learn about Rusts error handling and why expect/unwrap is bad to use
fn get_repository(url: &str) -> Result<Repository, git2::Error> {
    let path = format!(
        "repos/{}",
        url.split('/').last().expect("Not a valid repository link!")
    );
    let path = Path::new(&path);
    match path.exists() {
        true => Repository::open(path),
        false => {
            fs::create_dir_all(path).expect("Can't create a local repository directory!");
            Repository::clone(url, path)
        }
    }
}

fn repository(path: &str) -> Result<Repository, git2::Error> {
    match path.contains("github.com") {
        true => get_repository(path),
        false => Repository::open(path),
    }
}

// TODO: Reduce the size of this function
fn _expand_tree(repo: &Repository, tree: &git2::Tree) {
    for entry in tree.iter() {
        match entry.kind() {
            Some(git2::ObjectType::Blob) => {
                let blob = repo.find_blob(entry.id()).unwrap();
                println!("{:?}, {:?}", from_utf8(blob.content()).expect("Failed unwrapping the string!").len(), entry.name().unwrap());
            }
            Some(git2::ObjectType::Tree) => {
                let subtree = repo.find_tree(entry.id()).unwrap();
                _expand_tree(repo, &subtree);
            }
            _ => ()
        }
    }
}

fn commit(repository: &Repository) -> Result<Commit, git2::Error> {
    repository.head()?.peel_to_commit()
}

fn main() {
    // To get the diff between two trees one can look at blob pointers
    let repository = repository("https://github.com/kachayev/fn.py").expect("Couldn't open repository!");
    let _main_tree = repository.head().unwrap().peel_to_tree().unwrap();
    _expand_tree(&repository, &_main_tree);

    let mut commit = commit(&repository).unwrap();

    // Walk through the commit history and print the commit messages and trees
    loop {
        println!("Commit: {}", commit.summary().unwrap());

        // Get the tree for the commit
        let tree = commit.tree().expect("Couldn't get tree for commit!");
        println!("Tree: {}", tree.id());
        let diff = repository.diff_tree_to_tree(Some(&_main_tree), Some(&tree), None).unwrap();
        println!("Diff: {}", diff.stats().unwrap().deletions());
        diff.print(NameOnly, |delta, _, _| {
            println!("Delta: {:?}", delta);
            true
        }).unwrap();

        // Get the parent of the current commit
        let parent = match commit.parents().last() {
            Some(parent) => parent,
            None => break,
        };
        
        commit = parent;
    }
}
