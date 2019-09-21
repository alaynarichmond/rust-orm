extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate postgres;

use proc_macro::TokenStream;
use::syn::{parse_macro_input, DeriveInput};
use::syn::Data::Struct;
use syn::export::TokenStream2;
use quote::ToTokens;


#[proc_macro_derive(Relation)]
pub fn impl_get_table(input: TokenStream) -> TokenStream {
    // input: item deriving the trait (struct, enum, or union)
    // TokenStream: format in which the proc_macro library exposes the input
    // DeriveInput: format which the syn parser library needs

    // Parse the TokenStream into a DeriveInput
    let input = parse_macro_input!(input as DeriveInput);

    // Make sure that the macro input is a struct
    let fields = match input.data {
        Struct(s) => s.fields,
        _ => panic!()
    };

    /* Generate code to make a Table struct */
    let struct_ident = input.ident;
    let struct_name = struct_ident.to_string();
    let mut construct_table_code: Vec<TokenStream2> = Vec::new();

    // Generate code to initialize table
    let initialize_table_code = quote! {
        let mut table = Table {
            name: #struct_name.to_owned(),
            columns: Vec::new()
        };
    };
    construct_table_code.push(initialize_table_code);


    // Generate code to add columns
    for field in fields.into_iter() {
        // extract the field name and type from the struct
        let field_name = field.ident.expect("field must have name").to_string();
        let field_type = field.ty;

        // use them to create the column
        let add_column_code = quote! {
            table.columns.push(Column {
                name: #field_name.to_owned(),
               // typ: #field_type
            });
        };
        construct_table_code.push(add_column_code);
    }


    let impl_relation = quote! {
        impl Relation for #struct_ident {
            fn get_table() -> Table {
                #(#construct_table_code) *
                return table;
            }
        }
    };


    proc_macro::TokenStream::from(impl_relation)
}