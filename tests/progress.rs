#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test01_delegate.rs");
    t.pass("tests/test02_nested_modules.rs");
    t.pass("tests/test03_delegate_with_args.rs");
    t.pass("tests/test04_tuple.rs");
    t.pass("tests/test05_reference.rs");
    t.pass("tests/test06_lifetime.rs");
    t.pass("tests/test07_multiple_lifetimes.rs");
    t.pass("tests/test08_generics.rs");
    t.pass("tests/test09_multiple_generics.rs");
    t.pass("tests/test10_type_slice.rs");
    t.pass("tests/test11_generics_bound.rs");
    t.pass("tests/test12_generics_where.rs");
    t.pass("tests/test13_multiple_delegate_traits.rs");
    t.pass("tests/test14_multiple_fields.rs");
    t.pass("tests/test15_enum.rs");
    t.pass("tests/test16_enum_multiple_traits.rs");
    t.pass("tests/test17_child_type_is_generics.rs");
    t.pass("tests/test18_new_type.rs");
    t.pass("tests/test19_async_trait.rs");
    t.pass("tests/test20_async_trait_with_fn_lifetime.rs");
    t.pass("tests/test21_super_trait.rs");
    t.pass("tests/test22_enum_super_trait.rs");
    t.pass("tests/test23_super_trait_hand_written.rs");
}
