use git2::{Repository, Commit, Tree};

fn main() {
    // Open the repository located at the given path
    let repo = match Repository::open("/path/to/repo") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open repository: {}", e),
    };

    // Get the head commit
    let head = match repo.head() {
        Ok(reference) => reference,
        Err(e) => panic!("failed to get head reference: {}", e),
    };

    let mut commit = match head.peel_to_commit() {
        Ok(commit) => commit,
        Err(e) => panic!("failed to get commit: {}", e),
    };

    // Walk through the commit history and print the commit messages and trees
    loop {
        println!("Commit: {}", commit.summary().unwrap());

        // Get the tree for the commit
        let tree = match commit.tree() {
            Ok(tree) => tree,
            Err(e) => panic!("failed to get tree for commit: {}", e),
        };

        println!("Tree: {}", tree.id());

        // Get the parent of the current commit
        let parent = match commit.parent(0) {
            Ok(parent) => parent,
            Err(_) => break,
        };

        commit = parent;
    }
}
