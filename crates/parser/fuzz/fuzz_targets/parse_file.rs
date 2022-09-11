#![no_main]
use fe_common::files::SourceFileId;
use fe_parser::parse_file;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    _ = parse_file(SourceFileId::dummy_file(), data);
});
