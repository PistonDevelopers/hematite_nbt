#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(nbt_macros)]

#[derive(NbtFmt)]
struct Simple {
    #[nbt_field]
    names: String,
  //~^ ERROR `#[nbt_field]` requires a &str value.
  //~^^ ERROR `#[nbt_field]` requires a &str value.
}
