#[macro_use] mod macro_f;

use crate::common::{make_diag, make_lexer};
use rectangular::lexer::TType;

// common_test(function name, string of what you want to test, the type you expect)
test!(plain_identifier, "myvar", Identifier);
test!(identifier_starting_with_keyword, "lett", Identifier);

// Keywords
test!(keyword_true, "true", True);
test!(keyword_false, "false", False);

// Type keywords
test!(keyword_i8_type, "i8", I8Key);
test!(keyword_u8_type, "u8", U8Key);
test!(keyword_i16_type, "i16", I16Key);
test!(keyword_u16_type, "u16", U16Key);
test!(keyword_i32_type, "i32", I32Key);
test!(keyword_u32_type, "u32", U32Key);
test!(keyword_i64_type, "i64", I64Key);
test!(keyword_u64_type, "u64", U64Key);
test!(keyword_f32_type, "f32", F32Key);
test!(keyword_f64_type, "f64", F64Key);
test!(keyword_bool_type, "bool", BoolKey);
