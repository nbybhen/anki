// Copyright: Ankitects Pty Ltd and contributors
// License: GNU AGPL, version 3 or later; http://www.gnu.org/licenses/agpl.html

use camino::Utf8Path;

/// On Unix, just a normal path. On Windows, c:\foo\bar.txt becomes
/// /c:/foo/bar.txt, which msys rsync expects.
pub fn absolute_msys_path(path: &Utf8Path) -> String {
    let path = path.to_string();
    println!("Path: {path}");
    if !cfg!(windows) {
        return path;
    }

    // strip off \\? verbatim prefix, which things like rsync/ninja choke on
    let drive = &path.chars().nth(4).unwrap();
    // and \ -> /
    format!("/{drive}/{}", path[7..].replace('\\', "/"))
}

/// Converts backslashes to forward slashes
pub fn unix_path(path: &Utf8Path) -> String {
    path.as_str().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_conversion() {
        assert_eq!(String::from("c:/foo/bar.txt"), unix_path(Utf8Path::new(&absolute_msys_path(Utf8Path::new("c:\\foo\\bar.txt"))))) ;
        assert_eq!(String::from("/c:/system32/User/Desktop/projects/main.py"),
            unix_path(Utf8Path::new(&absolute_msys_path(Utf8Path::new("\\c:\\system32\\User\\Desktop\\projects\\main.py")))));
        assert_eq!(String::from("c:/") unix_path(Utf8Path::new(&absolute_msys_path(Utf8Path::new("c:\\")))))
    }
}
