// Imports.

// Long import.
use syntax::ast::{ItemForeignMod, ItemImpl, ItemMac, ItemMod, ItemStatic, ItemDefaultImpl};
use exceedingly::looooooooooooooooooooooooooooooooooooooooooooooooooooooooooong::import::path::{ItemA,
                                                                                                ItemB};

use list::{
    // Some item
    SomeItem /* Comment */, /* Another item */ AnotherItem /* Another Comment */, // Last Item
    LastItem
};

use test::{  Other          /* C   */  , /*   A   */ self  /*    B     */    };

use syntax::{self};
use {/* Pre-comment! */
     Foo, Bar /* comment */};
use Foo::{Bar, Baz};
pub use syntax::ast::{Expr_, Expr, ExprAssign, ExprCall, ExprMethodCall, ExprPath};
use syntax::some::{};

mod Foo {
    pub use syntax::ast::{
        ItemForeignMod,
        ItemImpl, 
        ItemMac,
        ItemMod,
        ItemStatic, 
        ItemDefaultImpl
    };

    mod Foo2 {
        pub use syntax::ast::{ItemForeignMod, ItemImpl, ItemMac, ItemMod, ItemStatic, self, ItemDefaultImpl};
    }
}

fn test() {
use Baz::*;
        use Qux;
}

// Simple imports
use  foo::bar::baz as baz ;
use bar::quux  as    kaas;
use  foo;
