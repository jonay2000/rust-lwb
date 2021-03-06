use rust_lwb::language;

#[rustfmt::skip]
mod stl;
mod types;

language!(STL at mod stl);

#[cfg(test)]
mod tests {
    use crate::STL;
    use rust_lwb::language::Language;
    use rust_lwb::sources::source_file::SourceFile;
    use rust_lwb::typechecker::TypeChecker;

    // #[test]
    // fn test_precedence() {
    //     let p1 = STL::parse(&SourceFile::new("3 + 5 - 6;", "main.stl"));
    //     let p2 = STL::parse(&SourceFile::new("5 - 6 + 3;", "main.stl"));
    //     dbg!(&p1);
    //     dbg!(&p2);
    //
    //     assert_eq!(p1, p2);
    // }

    macro_rules! test_stl {
        ($name: ident: $input: literal $($tt: tt)*) => {
            #[test]
            fn $name() {
                let file = SourceFile::new($input, "main.stl");

                macro_rules! parse_test_type {
                    (should not parse) => {
                        let res = STL::parse(&file);
                        assert!(res.is_err(), "{:?}", res.unwrap());
                    };
                    (should typecheck) => {
                        let ast = STL::parse(&file);
                        let tc = TypeChecker::new();
                        let tres = tc.check_types(ast, &());
                        assert!(tres.is_ok(), "{}", tres.unwrap_err());
                    };
                    (should not typecheck) => {
                        let ast = STL::parse(&file);
                        let tc = TypeChecker::new();
                        let tres = tc.check_types(ast, &());
                        assert!(tres.is_err());
                    }
                }
                parse_test_type!($($tt)*);
            }

        };
    }

    test_stl!(addition_ok: "3 + 3;" should typecheck);
    test_stl!(addition_err_1: "3 + true;" should not typecheck);
    test_stl!(addition_err_2: "true + 3;" should not typecheck);
    test_stl!(subtraction_ok: "2 - 3;" should typecheck);
    test_stl!(subtraction_err: "2 - true;" should not typecheck);
    test_stl!(if_ok: "
if a == 5 {
    a = a + 3;
}
    " should typecheck);
    test_stl!(if_err: "
if a + 5 {
    a = a + 3;
}
    " should not typecheck);

    test_stl!(compound: "true - (3 - 5);" should not typecheck);
    test_stl!(compound_ok: "3 - (3 - 5);" should typecheck);

    // test_stl!(empty_list: "a = [];" should typecheck);
}
