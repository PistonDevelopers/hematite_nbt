#![feature(plugin, custom_derive)]
#![plugin(nbt_macros)]

// Should throw the error twice --- once for each field.

#[derive(NbtFmt)]
      //~^ ERROR `NbtFmt` cannot yet be derived for enums.
      //~^^ ERROR `NbtFmt` cannot yet be derived for enums.
enum TestEnum {
    Variant1,
    Variant2
}
