// Match expressions.

fn foo() {
    // A match expression.
    match x {
        // Some comment.
        a => foo(),
        b if 0 < 42 => foo(),
        c => { // Another comment.
            // Comment.
            an_expression;
            foo()
        }
        // Perhaps this should introduce braces?
        Foo(ref bar) =>
            aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        Pattern1 | Pattern2 | Pattern3 => false,
        Paternnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn |
        Paternnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn => {
            blah
        }
        Patternnnnnnnnnnnnnnnnnnn |
        Patternnnnnnnnnnnnnnnnnnn |
        Patternnnnnnnnnnnnnnnnnnn |
        Patternnnnnnnnnnnnnnnnnnn => meh,

        Patternnnnnnnnnnnnnnnnnnn |
        Patternnnnnnnnnnnnnnnnnnn if looooooooooooooooooong_guard => meh,

        Patternnnnnnnnnnnnnnnnnnnnnnnnn |
        Patternnnnnnnnnnnnnnnnnnnnnnnnn if looooooooooooooooooooooooooooooooooooooooong_guard =>
            meh,

        // Test that earlier patterns can take the guard space
        (aaaa, bbbbb, ccccccc, aaaaa, bbbbbbbb, cccccc, aaaa, bbbbbbbb, cccccc, dddddd) |
        Patternnnnnnnnnnnnnnnnnnnnnnnnn if loooooooooooooooooooooooooooooooooooooooooong_guard => {}

        _ => {}
        ast::PathParameters::AngleBracketedParameters(ref data) if data.lifetimes.len() > 0 ||
                                                                   data.types.len() > 0 ||
                                                                   data.bindings.len() > 0 => {
        }
    }

    let whatever = match something {
        /// DOC COMMENT!
        Some(_) => 42,
        // Comment on an attribute.
        #[an_attribute]
        // Comment after an attribute.
        None => 0,
        #[rustfmt_skip]
        Blurb     =>     {                  }
    };
}
