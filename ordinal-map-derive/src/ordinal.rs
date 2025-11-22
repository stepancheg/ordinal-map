use quote::format_ident;
use syn::spanned::Spanned;

pub(crate) fn derive_ordinal(
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let span = input.span();
    let input = syn::parse2::<syn::DeriveInput>(input)?;

    let ident = input.ident;

    let from_ordinal_ordinal_var = format_ident!("ordinal");

    let (size, index_expr, from_index_expr) = match input.data {
        syn::Data::Struct(s) => {
            let s = StructGen { s };
            (
                s.ordinal_size()?.expr(),
                s.ordinal()?.expr(),
                s.from_ordinal(&from_ordinal_ordinal_var)?,
            )
        }
        syn::Data::Enum(e) => {
            let e = EnumGen { e };
            (
                e.ordinal_size()?.expr(),
                e.ordinal()?.expr(),
                e.from_ordinal(&from_ordinal_ordinal_var)?,
            )
        }
        syn::Data::Union(_) => {
            return Err(syn::Error::new(
                span,
                "Ordinal cannot be derived for unions",
            ))
        }
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let check_overflow = check_overflow();
    let check_zero_size = check_zero_size();

    Ok(syn::parse_quote_spanned! { span =>
        #[allow(clippy::all)]
        impl #impl_generics ordinal_map::Ordinal for #ident #ty_generics #where_clause {
            const ORDINAL_SIZE: usize = #size;

            fn ordinal(&self) -> usize {
                #check_overflow
                #index_expr
            }

            #[allow(unreachable_code, unused_mut)]
            fn from_ordinal(#from_ordinal_ordinal_var: usize) -> std::option::Option<Self> {
                #check_zero_size
                #from_index_expr
            }
        }
    })
}

fn check_overflow() -> syn::Stmt {
    syn::parse_quote! {
        // Make sure multiplication doesn't overflow.
        let _check_overflow = Self::ORDINAL_SIZE;
    }
}

fn check_zero_size() -> syn::Stmt {
    syn::parse_quote! {
        // - Make sure multiplication doesn't overflow by accessing the field
        // - Make sure we don't divide by zero in the implementation body
        if Self::ORDINAL_SIZE == 0 {
            return None;
        }
    }
}

fn field_vars(fields: &syn::Fields) -> impl Iterator<Item = syn::Ident> + '_ {
    fields.iter().enumerate().map(|(i, f)| match &f.ident {
        Some(ident) => ident.clone(),
        None => syn::Ident::new(&format!("f{}", i), f.span()),
    })
}

fn field_types(fields: &syn::Fields) -> impl Iterator<Item = &syn::Type> {
    fields.iter().map(|f| &f.ty)
}

enum SizeExpr {
    Const(u64),
    Syn(syn::Expr),
    Sum(Vec<SizeExpr>),
    Product(Vec<SizeExpr>),
}

impl SizeExpr {
    fn ordinal_size(ty: &syn::Type) -> SizeExpr {
        SizeExpr::Syn(syn::parse_quote_spanned! { ty.span() =>
            <#ty as ordinal_map::Ordinal>::ORDINAL_SIZE
        })
    }

    fn ordinal_expr(value: &syn::Expr) -> SizeExpr {
        SizeExpr::Syn(syn::parse_quote_spanned! { value.span() =>
            ordinal_map::Ordinal::ordinal(#value)
        })
    }

    fn product(exprs: impl IntoIterator<Item = SizeExpr>) -> SizeExpr {
        fn product_ignore_const(exprs: impl IntoIterator<Item = SizeExpr>) -> SizeExpr {
            let mut exprs = exprs.into_iter();
            let Some(first) = exprs.next() else {
                return SizeExpr::Const(1);
            };
            let Some(second) = exprs.next() else {
                return first;
            };
            SizeExpr::Product([first, second].into_iter().chain(exprs).collect())
        }

        let mut product = 1u64;
        let mut items = Vec::new();
        for expr in exprs {
            match expr {
                SizeExpr::Const(c) => product = product.checked_mul(c).expect("size overflow"),
                SizeExpr::Product(xs) => items.extend(xs),
                s @ (SizeExpr::Syn(_) | SizeExpr::Sum(_)) => items.push(s),
            }
        }
        match product {
            0 => SizeExpr::Const(0),
            1 => product_ignore_const(items),
            c => product_ignore_const([SizeExpr::Const(c)].into_iter().chain(items)),
        }
    }

    fn sum(exprs: impl IntoIterator<Item = SizeExpr>) -> SizeExpr {
        fn sum_ignore_const(exprs: impl IntoIterator<Item = SizeExpr>) -> SizeExpr {
            let mut exprs = exprs.into_iter();
            let Some(first) = exprs.next() else {
                return SizeExpr::Const(0);
            };
            let Some(second) = exprs.next() else {
                return first;
            };
            SizeExpr::Sum([first, second].into_iter().chain(exprs).collect())
        }

        let mut sum = 0u64;
        let mut items = Vec::new();
        for expr in exprs {
            match expr {
                SizeExpr::Const(c) => sum = sum.checked_add(c).expect("size overflow"),
                SizeExpr::Sum(xs) => items.extend(xs),
                s @ (SizeExpr::Syn(_) | SizeExpr::Product(_)) => items.push(s),
            }
        }
        match sum {
            0 => sum_ignore_const(items),
            c => sum_ignore_const([SizeExpr::Const(c)].into_iter().chain(items)),
        }
    }

    fn expr_const(value: u64) -> syn::Expr {
        let c = syn::LitInt::new(&format!("{value}usize"), proc_macro2::Span::call_site());
        syn::parse_quote! { #c }
    }

    fn expr(&self) -> syn::Expr {
        match self {
            SizeExpr::Const(c) => Self::expr_const(*c),
            SizeExpr::Syn(e) => e.clone(),
            SizeExpr::Sum(exprs) => {
                assert!(exprs.len() >= 2);
                let mut exprs = exprs.iter().map(|e| e.expr());
                let first = exprs.next().expect("At least one expr");
                syn::parse_quote! {
                    (
                        #first #( + #exprs )*
                    )
                }
            }
            SizeExpr::Product(exprs) => {
                assert!(exprs.len() >= 2);
                let mut exprs = exprs.iter().map(|e| e.expr());
                let first = exprs.next().expect("At least one expr");
                syn::parse_quote! {
                    (
                        #first #( * #exprs )*
                    )
                }
            }
        }
    }

    fn const_expr(&self) -> syn::Expr {
        match self {
            SizeExpr::Const(c) => Self::expr_const(*c),
            e => {
                let e = e.expr();
                syn::parse_quote! {
                    const {
                        #e
                    }
                }
            }
        }
    }
}

struct StructGen {
    s: syn::DataStruct,
}

fn struct_ordinal_size<'a>(types: impl IntoIterator<Item = &'a syn::Type>) -> SizeExpr {
    SizeExpr::product(types.into_iter().map(SizeExpr::ordinal_size))
}

fn struct_ordinal<'a>(
    field_expr_refs: impl IntoIterator<Item = &'a syn::Expr>,
    field_types: impl IntoIterator<Item = &'a syn::Type>,
    span: proc_macro2::Span,
) -> syn::Result<SizeExpr> {
    fn tuple_2_ordinal(a: SizeExpr, b: SizeExpr, b_count: SizeExpr) -> SizeExpr {
        SizeExpr::sum([SizeExpr::product([a, b_count]), b])
    }

    fn struct_ordinal_impl(
        field_expr_refs: &[&syn::Expr],
        field_types: &[&syn::Type],
        span: proc_macro2::Span,
    ) -> syn::Result<SizeExpr> {
        match (field_expr_refs, field_types) {
            ([], []) => Ok(SizeExpr::Const(0)),
            ([field_expr], [_]) => Ok(SizeExpr::ordinal_expr(field_expr)),
            ([first_expr, rem_exprs @ ..], [_first_ty, rem_tys @ ..]) => {
                let rem_count =
                    SizeExpr::product(rem_tys.iter().copied().map(|e| SizeExpr::ordinal_size(e)));
                let first_expr = SizeExpr::ordinal_expr(first_expr);
                let rem_expr = struct_ordinal_impl(rem_exprs, rem_tys, span)?;
                Ok(tuple_2_ordinal(first_expr, rem_expr, rem_count))
            }
            _ => Err(syn::Error::new(
                span,
                "Mismatched field_expr_refs and field_types",
            )),
        }
    }

    let expr = struct_ordinal_impl(
        field_expr_refs.into_iter().collect::<Vec<_>>().as_slice(),
        field_types.into_iter().collect::<Vec<_>>().as_slice(),
        span,
    )?;
    Ok(expr)
}

fn struct_from_ordinal<'a>(
    ordinal: &syn::Expr,
    field_vars: impl IntoIterator<Item = syn::Ident>,
    field_types: impl IntoIterator<Item = &'a syn::Type>,
    span: proc_macro2::Span,
    constructor: impl FnOnce(Vec<syn::Expr>) -> syn::Expr,
) -> syn::Result<syn::Expr> {
    let field_vars = Vec::from_iter(field_vars);
    let field_types = Vec::from_iter(field_types);

    let mut stmts: Vec<syn::Stmt> = Vec::new();
    stmts.push(syn::parse_quote_spanned! { span =>
        let mut rem = #ordinal;
    });
    for (field_var, field_ty) in field_vars.iter().zip(&field_types).rev() {
        let field_ordinal_size = SizeExpr::ordinal_size(field_ty).const_expr();
        stmts.extend([
            syn::parse_quote_spanned! { field_var.span() =>
                let #field_var = <#field_ty as ordinal_map::Ordinal>::from_ordinal(
                    rem % #field_ordinal_size).unwrap();
            },
            syn::parse_quote_spanned! { field_var.span() =>
                rem /= #field_ordinal_size;
            },
        ]);
    }
    stmts.push(syn::parse_quote_spanned! { span =>
        if rem != 0 {
            return None
        }
    });
    let build = constructor(
        field_vars
            .iter()
            .map(|f| syn::parse_quote! { #f })
            .collect(),
    );
    Ok(syn::parse_quote_spanned! { span =>
        {
            #( #stmts )*
            std::option::Option::Some(#build)
        }
    })
}

impl StructGen {
    fn field_ref_exprs(&self) -> impl Iterator<Item = syn::Expr> + '_ {
        self.s.fields.iter().enumerate().map(|(i, f)| {
            let i = syn::Index::from(i);
            match &f.ident {
                Some(ident) => syn::parse_quote_spanned! { ident.span() => &self.#ident },
                None => syn::parse_quote_spanned! { f.span() => &self.#i },
            }
        })
    }

    /// Generate `const ORDINAL_SIZE = ` RHS.
    fn ordinal_size(&self) -> syn::Result<SizeExpr> {
        Ok(struct_ordinal_size(field_types(&self.s.fields)))
    }

    /// Generate `fn ordinal(&self) -> usize` body.
    fn ordinal(&self) -> syn::Result<SizeExpr> {
        let field_types: Vec<_> = field_types(&self.s.fields).collect();
        let field_exprs: Vec<_> = self.field_ref_exprs().collect();
        struct_ordinal(
            &field_exprs,
            field_types.iter().copied(),
            self.s.struct_token.span,
        )
    }

    /// Generate `fn from_ordinal(ordinal: usize) -> Option<Self>` body.
    fn from_ordinal(&self, ordinal_var: &syn::Ident) -> syn::Result<syn::Expr> {
        let field_types: Vec<_> = field_types(&self.s.fields).collect();
        let field_vars: Vec<_> = field_vars(&self.s.fields).collect();
        struct_from_ordinal(
            &syn::parse_quote_spanned! { ordinal_var.span() => #ordinal_var },
            field_vars.iter().cloned(),
            field_types,
            self.s.struct_token.span,
            |exprs| match &self.s.fields {
                syn::Fields::Unit => syn::parse_quote_spanned! { self.s.struct_token.span =>
                    Self
                },
                syn::Fields::Unnamed(_) => {
                    syn::parse_quote_spanned! { self.s.struct_token.span =>
                        Self(
                            #( #exprs, )*
                        )
                    }
                }
                syn::Fields::Named(_) => {
                    syn::parse_quote_spanned! { self.s.struct_token.span =>
                        Self {
                            #( #field_vars: #exprs, )*
                        }
                    }
                }
            },
        )
    }
}

struct EnumGen {
    e: syn::DataEnum,
}

impl EnumGen {
    fn size_of_first_variants(&self, i: usize) -> syn::Result<SizeExpr> {
        let sizes = self
            .e
            .variants
            .iter()
            .take(i)
            .map(|v| struct_ordinal_size(v.fields.iter().map(|f| &f.ty)));
        Ok(SizeExpr::sum(sizes))
    }

    fn ordinal_size(&self) -> syn::Result<SizeExpr> {
        self.size_of_first_variants(self.e.variants.len())
    }

    fn ordinal(&self) -> syn::Result<SizeExpr> {
        if self.e.variants.is_empty() {
            // Special case because rust doesn't like `match &empty_enum {}`.
            return Ok(SizeExpr::Const(0));
        }

        let mut arms: Vec<syn::Arm> = Vec::new();
        for (i, variant) in self.e.variants.iter().enumerate() {
            let variant_name = &variant.ident;
            let vars = Vec::from_iter(field_vars(&variant.fields));
            let vars = vars.as_slice();
            let size_of_first_variants = self.size_of_first_variants(i)?;
            let bind: syn::Pat = match &variant.fields {
                syn::Fields::Unit => syn::parse_quote_spanned! {
                    variant.span() =>
                    Self::#variant_name
                },
                syn::Fields::Unnamed(_) => {
                    syn::parse_quote_spanned! {
                        variant.span() =>
                        Self::#variant_name( #( #vars ),* )
                    }
                }
                syn::Fields::Named(_) => {
                    syn::parse_quote_spanned! {
                        variant.span() =>
                        Self::#variant_name { #( #vars ),* }
                    }
                }
            };
            let struct_ordinal = struct_ordinal(
                vars.iter()
                    .map(|v| syn::parse_quote! { #v })
                    .collect::<Vec<_>>()
                    .as_slice(),
                field_types(&variant.fields),
                variant.span(),
            )?;
            let value = SizeExpr::sum([size_of_first_variants, struct_ordinal]).expr();
            arms.push(syn::parse_quote_spanned! {
                variant.span() =>
                #bind => {
                    #value
                }
            })
        }
        Ok(SizeExpr::Syn(syn::parse_quote_spanned! {
            self.e.enum_token.span =>
            match self {
                #( #arms, )*
            }
        }))
    }

    fn from_ordinal(&self, ordinal_var: &syn::Ident) -> syn::Result<syn::Expr> {
        let mut stmts: Vec<syn::Stmt> = Vec::new();
        for (i, variant) in self.e.variants.iter().enumerate() {
            let size_of_first_variants_before = self.size_of_first_variants(i)?.const_expr();
            let size_of_first_variants_including = self.size_of_first_variants(i + 1)?.const_expr();
            let rem_ordinal = syn::parse_quote_spanned! { variant.span() =>
                #ordinal_var - #size_of_first_variants_before
            };
            let variant_name = &variant.ident;
            let struct_from_ordinal = struct_from_ordinal(
                &rem_ordinal,
                field_vars(&variant.fields),
                field_types(&variant.fields),
                variant.span(),
                |exprs| match &variant.fields {
                    syn::Fields::Unit => syn::parse_quote_spanned! { variant.span() =>
                        Self::#variant_name
                    },
                    syn::Fields::Unnamed(_) => {
                        syn::parse_quote_spanned! { variant.span() =>
                            Self::#variant_name( #( #exprs ),* )
                        }
                    }
                    syn::Fields::Named(_) => {
                        syn::parse_quote_spanned! { variant.span() =>
                            Self::#variant_name { #( #exprs ),* }
                        }
                    }
                },
            )?;
            stmts.push(syn::parse_quote_spanned! {
                variant.span() =>
                if #ordinal_var < #size_of_first_variants_including {
                    return #struct_from_ordinal;
                }
            })
        }
        Ok(syn::parse_quote_spanned! { self.e.enum_token.span =>
            {
                #( #stmts )*
                let _ignore = #ordinal_var;
                std::option::Option::None
            }
        })
    }
}

fn impl_ordinal_for_tuple_n(n: u32) -> syn::Result<syn::ItemImpl> {
    let ordinal_var = format_ident!("ordinal");
    let params: Vec<_> = (0..n)
        .map(|i| format_ident!("{}", char::try_from('A' as u32 + i).unwrap()))
        .collect();
    let vars: Vec<_> = (0..n)
        .map(|i| format_ident!("{}", char::try_from('a' as u32 + i).unwrap()))
        .collect();
    let field_types: Vec<syn::Type> = params.iter().map(|p| syn::parse_quote! { #p }).collect();
    let field_ref_exprs: Vec<syn::Expr> = (0..n)
        .map(|i| {
            let i = syn::Index::from(i as usize);
            syn::parse_quote! { &self.#i }
        })
        .collect();
    let ordinal_size = struct_ordinal_size(&field_types).expr();
    let ordinal = struct_ordinal(
        &field_ref_exprs,
        &field_types,
        proc_macro2::Span::call_site(),
    )?
    .expr();
    let from_ordinal = struct_from_ordinal(
        &syn::parse_quote_spanned! { ordinal_var.span() => #ordinal_var },
        vars,
        &field_types,
        proc_macro2::Span::call_site(),
        |exprs| syn::parse_quote! { ( #( #exprs, )* ) },
    )?;
    let check_overflow = check_overflow();
    let check_zero_size = check_zero_size();
    Ok(syn::parse_quote! {
        impl< #( #params: ordinal_map::Ordinal, )* > ordinal_map::Ordinal for ( #( #params, )* ) {
            const ORDINAL_SIZE: usize = #ordinal_size;

            fn ordinal(&self) -> usize {
                #check_overflow
                #ordinal
            }

            fn from_ordinal(#ordinal_var: usize) -> std::option::Option<Self> {
                #check_zero_size
                #from_ordinal
            }
        }
    })
}

pub(crate) fn impl_ordinal_for_tuple(
    _input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let tuples: Vec<_> = (0..=16)
        .map(impl_ordinal_for_tuple_n)
        .collect::<syn::Result<_>>()?;
    Ok(quote::quote! {
        #( #tuples )*
    })
}
