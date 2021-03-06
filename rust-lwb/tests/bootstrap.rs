use rust_lwb::language::Language;
use rust_lwb::parser::syntax_file::SyntaxFile;
use rust_lwb::sources::source_file::SourceFile;

#[test]
fn test_bootstrap() {
    let sf = SourceFile::open("../rust-lwb-bootstrap/syntax-file.syntax").unwrap();
    let res = SyntaxFile::try_parse(&sf);

    assert!(res.is_ok(), "{}", res.err().unwrap())
}
