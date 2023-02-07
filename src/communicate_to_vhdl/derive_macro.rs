use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};
use crate::communicate_to_vhdl::Vhdlizable;


#[proc_macro_derive(Vhdlizable)]
pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();


    let expanded = quote! {
        // The generated impl.
        impl #impl_generics Vhdlizable for #name #ty_generics #where_clause {


            fn get_necessary_bits() -> usize{
                0
            }

            fn get_bit_representation(&self) -> Vec<bool>{
                Vec::new()
            }

            fn construct_from_bits(data: &Vec<bool>) -> Result<Self,Error> where Self: Sized{
                panic!()
            }

            fn get_vhd_construction_code(variable_name: &str ,start_index: usize) -> String{
                String::new()
            }

            fn get_vhd_declaration_code(variable_name: &str ) -> String{
                String::new()
            }

            fn get_vhd_deconstruction_code(variable_name: &str ,start_index: usize) -> String{
                String::new()
            }

        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)

}



// Add a bound `T: HeapSize` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(heapsize::HeapSize));
        }
    }
    generics
}