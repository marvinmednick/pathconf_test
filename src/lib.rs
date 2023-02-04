use syn::DeriveInput;

// What this code needs to is:
//   create a function that returns a dict (or temporarity a Hashmap) where
//   the key is the String of the pathconf enum, and the value is the Value of the num
//   This will run onetime at python startup to poplulate the dict something like
//
//    #[pyattr]
//    fn environ(vm: &VirtualMachine) -> PyDictRef {
//        use ffi_ext::osstringext;
//
//       let environ = vm.ctx.new_dict();
//        for (key, value) in env::vars_os() {
//`            let key: pyobjectref = vm.ctx.new_bytes(key.into_vec()).into();
//`            let value: pyobjectref = vm.ctx.new_bytes(value.into_vec()).into();
//`            environ.set_item(&*key, value, vm).unwrap();
//`        }
//`
//`        environ
//`    }
//`
//    but in this case the function built be populated for each variant of the enum
//    one by one as such
//    key = "EnumVariat: (as a string)
//    value = Enum::EnumVariant as u32
//

#[proc_macro_derive(EnumToDict)]
pub fn derive_enum_to_dict(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input: DeriveInput = syn::parse(input).unwrap();

    impl_enum_to_dict(parsed_input)
}

fn impl_enum_to_dict(input: DeriveInput) -> proc_macro::TokenStream {
    let struct_name = input.ident;
    let struct_string_name = format!("{struct_name}");
    let dict_name = quote::format_ident!("{}_names", struct_string_name.to_lowercase());

    // get the list of fields from the structure
    let variants = if let syn::Data::Enum(syn::DataEnum { ref variants, .. }) = input.data {
        variants
    } else {
        // this dervive (builder) only supports structures at this time
        unimplemented!();
    };

    let mut entries = Vec::<proc_macro2::TokenStream>::new();
    for v in variants {
        let key = format!("{}", v.ident);
        let key_ident = quote::format_ident!("{}", v.ident);

        let new_entry = quote::quote! {
            println!("{} : {}", #key, #struct_name::#key_ident as u32);
        };

        entries.push(new_entry);
    }
    //    eprintln!("Entries is {:#?}", entries);

    quote::quote! {
            pub fn #dict_name() {
                #(#entries)*
            }
    }
    .into()
}
