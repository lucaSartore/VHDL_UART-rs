use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics,
};




#[proc_macro_derive(Vhdlizable)]
pub fn derive_vhdliizable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: HeapSize` to every type parameter T.
    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();


    let necessary_bits = sum_necessary_bits(&input.data);

    let bit_rapresentation = concat_bit_representation(&input.data);

    let declaration_code = generate_delcaration_code(&input.data);

    let construction_code = generate_construction_code(&input.data);

    let deconstruction_code = generate_deconstruction_code(&input.data);

    let recreate_from_bits_code = generate_recursive_reconstruction(&input.data);

    let expanded = quote! {

        // The generated impl.
        impl #impl_generics Vhdlizable for #name #ty_generics #where_clause {

            #[allow(dead_code)]
            fn get_necessary_bits() -> usize{
                #necessary_bits
            }
            #[allow(dead_code)]
            fn get_bit_representation(&self) -> Vec<bool>{
                #bit_rapresentation
            }
            #[allow(dead_code)]
            fn construct_from_bits(data: &[bool]) -> Result<Self,std::io::Error> where Self: Sized{

                if data.len() != Self::get_necessary_bits(){
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "Length of input incompatible with length of output"));
                };

                #recreate_from_bits_code
            }
            #[allow(dead_code)]
            fn get_vhd_construction_code(variable_name: &str ,start_index: usize) -> String{
                #construction_code
            }
            #[allow(dead_code)]
            fn get_vhd_declaration_code(variable_name: &str ) -> String{
                #declaration_code
            }
            #[allow(dead_code)]
            fn get_vhd_deconstruction_code(variable_name: &str ,start_index: usize) -> String{
                #deconstruction_code
            }

        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)

}

// Add a bound `T: Vhdlizable` to every type parameter T.
fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(Vhdlizable));
        }
    }
    generics
}

// Generate an expression to sum up the heap size of each field.
fn sum_necessary_bits(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse = fields.named.iter().map(|f| {
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>
                            #type_::get_necessary_bits()
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unnamed(_) => {
                    unimplemented!("Impossible to derive this trait on unnamed Type")
                }
                Fields::Unit => {
                    unimplemented!("Impossible to derive this trait on unit Type")
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn concat_bit_representation(data: &Data) -> TokenStream {

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse: Vec<_> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            self.#name.get_bit_representation()
                        }
                    }).collect();

                    quote! {
                        let mut v = vec![false;0];
                        #(v.extend(#recurse);)*
                        v
                    }
                }
                Fields::Unnamed(_) => {
                    unimplemented!("Impossible to derive this trait on unnamed Type")
                }
                Fields::Unit => {
                    unimplemented!("Impossible to derive this trait on unit Type")
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn generate_delcaration_code(data: &Data) -> TokenStream {

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse: Vec<_> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>
                            //self.#name.get_delcaration_code("name")
                            #type_::get_vhd_declaration_code(
                                &format!("{}_{}",variable_name,stringify!(#name))[..]
                            )
                        }
                    }).collect();


                    quote! {
                        let mut s = String::new();
                        #(s.push_str(&(#recurse)[..]);)*
                        s
                    }
                }
                Fields::Unnamed(_) => {
                    unimplemented!("Impossible to derive this trait on unnamed Type")
                }
                Fields::Unit => {
                    unimplemented!("Impossible to derive this trait on unit Type")
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}


fn generate_construction_code(data: &Data) -> TokenStream {

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse: Vec<_> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>
                            #type_::get_vhd_construction_code(
                                &format!("{}_{}",variable_name,stringify!(#name))[..],
                                start_from
                            )
                        }
                    }).collect();

                    let sizes: Vec<_> = fields.named.iter().map(|f| {
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>
                            #type_::get_necessary_bits()
                        }
                    }).collect();

                    quote! {
                        let mut start_from = start_index;
                        let mut s = String::new();
                        #(
                            s.push_str(&(#recurse)[..]);
                            start_from += #sizes;
                        )*
                        s
                    }
                }
                Fields::Unnamed(_) => {
                    unimplemented!("Impossible to derive this trait on unnamed Type")
                }
                Fields::Unit => {
                    unimplemented!("Impossible to derive this trait on unit Type")
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn generate_deconstruction_code(data: &Data) -> TokenStream {

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let recurse: Vec<_> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>
                            #type_::get_vhd_deconstruction_code(
                                &format!("{}_{}",variable_name,stringify!(#name))[..],
                                start_from
                            )
                        }
                    }).collect();

                    let sizes: Vec<_> = fields.named.iter().map(|f| {
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>
                            #type_::get_necessary_bits()
                        }
                    }).collect();

                    quote! {
                        let mut start_from = start_index;
                        let mut s = String::new();
                        #(
                            s.push_str(&(#recurse)[..]);
                            start_from += #sizes;
                        )*
                        s
                    }
                }
                Fields::Unnamed(_) => {
                    unimplemented!("Impossible to derive this trait on unnamed Type")
                }
                Fields::Unit => {
                    unimplemented!("Impossible to derive this trait on unit Type")
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

#[allow(dead_code)]
fn generate_recursive_reconstruction(data: &Data) -> TokenStream {

    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let mut recurse: Vec<_> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        let type_ = &f.ty;
                        quote_spanned! {f.span()=>

                            let n = #type_::get_necessary_bits();

                            let v = &data[counter..(counter+n)];

                            let #name: #type_ = #type_::construct_from_bits(v)?;
                        }
                    }).collect();

                    let names: Vec<_> = fields.named.iter().map(|f| {
                        let name = &f.ident;
                        quote_spanned! {f.span()=>
                            #name
                        }
                    }).collect();


                    let index_to_remove = recurse.len();
                    let last_elemend;
                    if index_to_remove > 0 {
                        last_elemend = recurse.remove(index_to_remove-1);

                        quote! {

                            let mut counter = 0_usize;

                            #(
                                #recurse
                                counter += n;
                            )*
                            #last_elemend

                            Ok(Self{
                                #(
                                #names,
                                )*
                            })
                        }
                    }else{
                        quote! {
                            Ok(
                                Self{}
                            )
                        }
                    }
                }
                Fields::Unnamed(_) => {
                    unimplemented!("Impossible to derive this trait on unnamed Type")
                }
                Fields::Unit => {
                    unimplemented!("Impossible to derive this trait on unit Type")
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

