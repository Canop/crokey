#![no_std]
#![no_implicit_prelude]

#[allow(dead_code)]
fn hygiene() {
    ::crokey::key!(M);
    ::crokey::key!(ctrl-c);
    ::crokey::key!(alt-shift-ctrl-']');
}
