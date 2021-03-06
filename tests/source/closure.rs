// Closures

fn main() {
    let square = ( |i:  i32 | i  *  i );

    let commented = |/* first */ a /*argument*/, /* second*/ b: WithType /* argument*/, /* ignored */ _ |
        (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbbbb);

    let commented = |/* first */ a /*argument*/, /* second*/ b: WithType /* argument*/, /* ignored */ _ |
        (aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb);

    let block_body = move   |xxxxxxxxxxxxxxxxxxxxxxxxxxxxx,  ref  yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy| {
            xxxxxxxxxxxxxxxxxxxxxxxxxxxxx + yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
        };

    let loooooooooooooong_name = |field| {
             // TODO(#27): format comments.
             if field.node.attrs.len() > 0 { field.node.attrs[0].span.lo
             } else {
                 field.span.lo
             }};

    let block_me = |field| if true_story() { 1 } else { 2 };

    let unblock_me = |trivial| {
                         closure()
                     };

    let empty = |arg|    {};

    let test = |  | { do_something(); do_something_else(); };

    let arg_test = |big_argument_name, test123| looooooooooooooooooong_function_naaaaaaaaaaaaaaaaame();

    let arg_test = |big_argument_name, test123| {looooooooooooooooooong_function_naaaaaaaaaaaaaaaaame()};

    |arg1, arg2, _, _, arg3, arg4| { let temp = arg4 + arg3;
                                     arg2 * arg1 - temp }
}
