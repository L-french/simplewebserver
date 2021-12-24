use simplewebserver;
use assert_cmd::Command;

#[test]
fn serve_two_files() {
    Command::cargo_bin("simplewebserver")
        .unwrap()
        // using relative paths within assert_cmd causes platform-dependent behavior,
        // and is not recommended -- TODO
        .current_dir("./tests/cli-test-env")
        .arg("-D")
        .arg("file1-1.txt")
        .arg("file1-2.txt")
        .assert()
        .stdout("FILES: [\"file1-1.txt\", \"file1-2.txt\"]\n");
}

#[test]
fn serve_simple_directory() {
    Command::cargo_bin("simplewebserver")
        .unwrap()
        // using relative paths within assert_cmd causes platform-dependent behavior,
        // and is not recommended -- TODO
        .current_dir("./tests/cli-test-env/dir2/dir3")
        .arg("-D")
        .arg("-r")
        .arg("./")
        .assert()
        .stdout("FILES: [\"file3-1.txt\"]\n");
}

#[test]
fn serve_nested_directory() {
    Command::cargo_bin("simplewebserver")
        .unwrap()
        // using relative paths within assert_cmd causes platform-dependent behavior,
        // and is not recommended -- TODO
        .current_dir("./tests/cli-test-env/dir2")
        .arg("-D")
        .arg("-r")
        .arg("./")
        .assert()
        .stdout("FILES: [\"file2-1.txt\", \"dir3/file3-1.txt\"]\n");
}
