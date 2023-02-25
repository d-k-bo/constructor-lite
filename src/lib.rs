use darling::{ast::Data, util::Flag, Error, FromDeriveInput, FromField};
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Generics, Ident, Path, Type, Visibility};

#[derive(Debug, FromField)]
#[darling(attributes(constructor), and_then = "Self::not_both")]
struct Field {
    ident: Option<Ident>,
    ty: Type,

    required: Flag,
    default: Flag,
}
impl Field {
    fn not_both(self) -> darling::Result<Self> {
        if self.required.is_present() && self.default.is_present() {
            Err(
                Error::custom("Field cannot use `required` and `default`at the same time.")
                    .with_span(&self.default),
            )
        } else {
            Ok(self)
        }
    }
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(constructor), supports(struct_named))]
struct ConstructorLite {
    vis: Visibility,
    ident: Ident,
    generics: Generics,
    data: Data<(), Field>,

    visibility: Option<Visibility>,
    name: Option<Ident>,
}
impl ConstructorLite {
    fn constructor(&self) -> darling::Result<proc_macro::TokenStream> {
        let Self {
            vis,
            ident,
            generics,
            data,
            visibility,
            name,
        } = self;

        let Data::Struct(fields) = data else {
            return Err(Error::custom("ConstructorLite supports only structs."));
        };

        let mut arguments = Vec::new();
        let mut required_field_idents = Vec::new();
        let mut optional_field_idents = Vec::new();

        for Field {
            ident,
            ty,
            required,
            default,
        } in fields.iter()
        {
            if required.is_present() {
                arguments.push(quote!(#ident: #ty));
                required_field_idents.push(ident);
                continue;
            }
            if default.is_present() {
                optional_field_idents.push(ident);
                continue;
            }

            if let Type::Path(ty) = &ty {
                if path_is_option(&ty.path) {
                    optional_field_idents.push(ident);
                } else {
                    arguments.push(quote!(#ident: #ty));
                    required_field_idents.push(ident);
                }
            }
        }

        let vis = visibility.as_ref().unwrap_or(vis);
        let name: Ident = name
            .clone()
            .unwrap_or_else(|| Ident::new("new", Span::call_site()));

        let constructor = quote!(
            impl #generics #ident #generics {
                #vis fn #name ( #( #arguments ),* ) -> Self {
                    Self {
                        #(
                            #required_field_idents,
                        )*
                        #(
                            #optional_field_idents: Default::default(),
                        )*
                    }
                }
            }
        );

        Ok(constructor.into())
    }
}

fn path_is_option(path: &Path) -> bool {
    // Option<T>
    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.first().unwrap().ident == "Option"
    {
        return true;
    }

    let mut segments = path.segments.iter();

    // core::option::Option<T>
    // ::core::option::Option<T>
    // std::option::Option<T>
    // ::std::option::Option<T>
    segments
        .next()
        .map(|seg| seg.ident == "core" || seg.ident == "std")
        .unwrap_or(false)
        && segments
            .next()
            .map(|seg| seg.ident == "option")
            .unwrap_or(false)
        && segments
            .next()
            .map(|seg| seg.ident == "Option")
            .unwrap_or(false)
}

#[proc_macro_derive(ConstructorLite, attributes(constructor))]
pub fn constructor_lite_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ConstructorLite::from_derive_input(&parse_macro_input!(input as syn::DeriveInput))
        .and_then(|constructor| constructor.constructor())
        .unwrap_or_else(|e| e.write_errors().into())
}
