#[macro_use] mod macro_f;

use crate::common::{make_diag, make_lexer};
use rectangular::lexer::TType;

// common_test(function name, string of what you want to test, the type you expect)
lexer_ctest!(plain_identifier, "myvar", Identifier);
lexer_ctest!(identifier_starting_with_keyword, "lett", Identifier);

// Keywords
lexer_ctest!(keyword_true, "true", True);
lexer_ctest!(keyword_false, "false", False);

// Type keywords
lexer_ctest!(keyword_i8_type, "i8", I8Key);
lexer_ctest!(keyword_u8_type, "u8", U8Key);
lexer_ctest!(keyword_i16_type, "i16", I16Key);
lexer_ctest!(keyword_u16_type, "u16", U16Key);
lexer_ctest!(keyword_i32_type, "i32", I32Key);
lexer_ctest!(keyword_u32_type, "u32", U32Key);
lexer_ctest!(keyword_i64_type, "i64", I64Key);
lexer_ctest!(keyword_u64_type, "u64", U64Key);
lexer_ctest!(keyword_f32_type, "f32", F32Key);
lexer_ctest!(keyword_f64_type, "f64", F64Key);
lexer_ctest!(keyword_bool_type, "bool", BoolKey);
