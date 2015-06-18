#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

#[derive(NbtFmt)] //~ ERROR `NbtFmt` cannot yet be derived for enums.
enum TestEnum {
    Variant1,
    Variant2
}
