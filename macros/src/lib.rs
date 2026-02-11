use darling::{Error, FromDeriveInput, FromField, ast::Data};
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, Meta, Path, Token, Type, parse::Parse, punctuated::Punctuated};

#[derive(Debug, FromField)]
#[darling(attributes(game))]
struct GameField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    #[darling(default)]
    base: bool,
    #[darling(default)]
    component: bool,
    #[darling(default)]
    object: bool,
}

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(game), supports(struct_named))]
struct GameReceiver {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<darling::util::Ignored, GameField>,
    #[darling(default, with = "parse_subscriptions")]
    subscribe: Vec<Subscription>,
}

#[derive(Debug)]
struct Subscription{
    handler: Ident,
    event_type: Path
}

impl Parse for Subscription{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let handler: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let event_type: Path = input.parse()?;
        Ok(Subscription{handler, event_type})
    }
}

fn parse_subscriptions(meta: &Meta) -> Result<Vec<Subscription>, Error>{
    let list = match meta {
        Meta::List(list) => list,
        _ => return Err(Error::custom("Use formato: subscribe(metodo: Tipo)"))
    };

    let parser = Punctuated::<Subscription, Token![,]>::parse_terminated;

    let subs = list.parse_args_with(parser).map_err(|e| Error::custom(e.to_string()).with_span(meta))?;
    Ok(subs.into_iter().collect())
}

#[proc_macro_derive(GameObject, attributes(game))]
pub fn scene_tree(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let receiver = match GameReceiver::from_derive_input(&input) {
        Ok(v) => v,
        Err(e) => return e.write_errors().into(),
    };

    let mut subscribe_arms = Vec::new();
    for sub in receiver.subscribe{
        let event_ty = &sub.event_type;
        let handler_ident = &sub.handler;
        subscribe_arms.push(quote! {
            if let Some(payload) = any_event.downcast_ref::<#event_ty>(){
                self.#handler_ident(ctx, payload);
            }
        });
    }

    let struct_name = &receiver.ident;

    let fields = receiver.data.take_struct().unwrap();

    let mut base_field = None;
    let mut component_fields = Vec::new();
    let mut object_fields = Vec::new();
    let mut bounds = Vec::new();

    for field in fields.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;
        if field.base {
            if base_field.is_some() {
                return syn::Error::new_spanned(
                    ident,
                    "Apenas um campo pode ser marcado como base",
                )
                .to_compile_error()
                .into();
            }
            if type_is_base(ty) {
                base_field = Some(ident.clone());
            } else {
                return syn::Error::new_spanned(ty, "Precisa ser do tipo Transform2D")
                    .to_compile_error()
                    .into();
            }
        }else if field.component {
            component_fields.push(ident.clone());
            bounds.push(quote! {#ty: ::milim_2d::Component});
        }else if field.object {
            object_fields.push(ident.clone());
            bounds.push(quote! {#ty: ::milim_2d::GameObject + ::milim_2d::GameObjectDispatch});
        }
    }
    let (impl_generics, ty_generics, where_clause) = receiver.generics.split_for_impl();

    let where_tokens = if let Some(wc) = where_clause {
        quote! {#wc,  Self: ::milim_2d::GameObject, #(#bounds),*}
    } else {
        quote! {where  Self: ::milim_2d::GameObject, #(#bounds),*}
    };
    quote! {
        impl #impl_generics ::milim_2d::GameObjectBase for #struct_name #ty_generics {
            fn base(&self) -> &::milim_2d::Base {
                &self.#base_field
            }

            fn base_mut(&mut self) -> &mut ::milim_2d::Base {
                &mut self.#base_field
            }
        }
        impl #impl_generics ::milim_2d::GameObjectDispatch for #struct_name #ty_generics #where_tokens {
            fn is_pending_removal(&self) -> bool{
                self.base().pending_removal
            }
            fn dispatch_start(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base) {
                let inherit = !self.#base_field.top_level;
                self.#base_field.transform.apply_parent(&parent_base.transform, inherit);

                #(self.#component_fields.start(ctx, &mut self.#base_field);)*
                
                self.start(ctx);
                #( self.#object_fields.dispatch_start(ctx, &self.#base_field); )*
            }
            fn dispatch_message(&mut self, ctx: &mut ::milim_2d::EngineContext){
                if let Some(msgs) = ctx.mailbox.remove(&self.base().id){
                    for msg in msgs {
                        if let Some(message) = msg.downcast_ref::<<Self as ::milim_2d::GameObject>::Message>(){
                            self.on_message(ctx, message)
                        } else{
                            println!("Tipo de evento incompativel recebido");
                        }
                    }
                }
                #(self.#object_fields.dispatch_message(ctx);)*
            }
            fn dispatch_event(&mut self, ctx: &mut ::milim_2d::EngineContext, event: &::milim_2d::GlobalEvent){
                
                if let ::milim_2d::GlobalEvent::Broadcast(any_event) = event{
                    #(#subscribe_arms)*
                }
                
                #(self.#component_fields.on_event(ctx, &mut self.#base_field, event);)*
                #(self.#object_fields.dispatch_event(ctx, event);)*
            }
            fn dispatch_update(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base, delta: f32) {
                let inherit = !self.#base_field.top_level;
                self.#base_field.transform.apply_parent(&parent_base.transform, inherit);

                #(self.#component_fields.update(ctx, &mut self.#base_field, delta);)*

                self.update(ctx, delta);


                #( self.#object_fields.dispatch_update(ctx, &self.#base_field, delta); )*
            }
            fn dispatch_late_update(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base, delta: f32) {
                let inherit = !self.#base_field.top_level;
                self.#base_field.transform.apply_parent(&parent_base.transform, inherit);
                #(self.#component_fields.late_update(ctx, &mut self.#base_field, delta);)*
                
                self.late_update(ctx, delta);

                #( self.#object_fields.dispatch_late_update(ctx, &self.#base_field, delta); )*
            }
            fn dispatch_fixed_update(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base) {
                let inherit = !self.#base_field.top_level;
                self.#base_field.transform.apply_parent(&parent_base.transform, inherit);

                #(self.#component_fields.fixed_update(ctx, &mut self.#base_field);)*
                
                self.fixed_update(ctx);


                #( self.#object_fields.dispatch_fixed_update(ctx, &self.#base_field); )*
            }
            fn dispatch_draw(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base) {
                let inherit = !self.#base_field.top_level;
                self.#base_field.transform.apply_parent(&parent_base.transform, inherit);
                #(self.#component_fields.draw(ctx, &self.#base_field);)*
                
                #( self.#object_fields.dispatch_draw(ctx, &self.#base_field); )*
            }
            fn dispatch_destroy(&mut self, ctx: &mut ::milim_2d::EngineContext) {
                #(self.#component_fields.destroy(ctx);)*
                self.destroy(ctx);
                #( self.#object_fields.dispatch_destroy(ctx); )*
            }
        }
    }
    .into()
}

fn type_is_base(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) => {
            if let Some(seg) = type_path.path.segments.last() {
                if seg.ident == "Base" {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

#[proc_macro_derive(Scene)]
pub fn scene_dispatch_derive(input: TokenStream) -> TokenStream{
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let data = match input.data {
        syn::Data::Enum(d) => d,
        _=> return syn::Error::new_spanned(name, "Scene so pode ser utilizado em enum").into_compile_error().into(),
    };

    let mut match_arms_start = Vec::new();
    let mut match_arms_update = Vec::new();
    let mut match_arms_late = Vec::new();
    let mut match_arms_fixed = Vec::new();
    let mut match_arms_draw = Vec::new();
    let mut match_arms_destroy = Vec::new();
    let mut match_arms_event = Vec::new();
    let mut match_arms_message = Vec::new();
    let mut match_arms_pending = Vec::new();

    for variant in data.variants{
        let ident = variant.ident;
        match_arms_start.push(quote! {
            Self::#ident(inner) => inner.dispatch_start(ctx, parent_base),
        });
        match_arms_update.push(quote! {
            Self::#ident(inner) => inner.dispatch_update(ctx, parent_base, delta),
        });
        match_arms_late.push(quote! {
            Self::#ident(inner) => inner.dispatch_late_update(ctx, parent_base, delta),
        });
        match_arms_fixed.push(quote! {
            Self::#ident(inner) => inner.dispatch_fixed_update(ctx, parent_base),
        });
        match_arms_draw.push(quote! {
            Self::#ident(inner) => inner.dispatch_draw(ctx, parent_base),
        });
        match_arms_destroy.push(quote! {
            Self::#ident(inner) => inner.dispatch_destroy(ctx),
        });
        match_arms_event.push(quote! {
            Self::#ident(inner) => inner.dispatch_event(ctx, event),
        });
        match_arms_message.push(quote! {
            Self::#ident(inner) => inner.dispatch_message(ctx),
        });
        match_arms_pending.push(quote! {
            Self::#ident(inner) => inner.is_pending_removal(),
        });
    }
    quote! {
        impl ::milim_2d::GameObjectDispatch for #name {
            fn is_pending_removal(&self) -> bool{
                match self{
                    #(#match_arms_pending)*
                }
            }
            fn dispatch_start(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base) {
                match self{
                    #(#match_arms_start)*
                }
            }
            fn dispatch_event(&mut self, ctx: &mut ::milim_2d::EngineContext, event: &::milim_2d::GlobalEvent){
                match self{
                    #(#match_arms_event)*
                }
            }
            fn dispatch_message(&mut self, ctx: &mut ::milim_2d::EngineContext){
                match self{
                    #(#match_arms_message)*
                }
            }
            fn dispatch_update(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base, delta: f32) {
                match self{
                    #(#match_arms_update)*
                }
            }
            fn dispatch_late_update(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base, delta: f32) {
                match self{
                    #(#match_arms_late)*
                }
            }
            fn dispatch_fixed_update(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base) {
                match self{
                    #(#match_arms_fixed)*
                }
            }
            fn dispatch_draw(&mut self, ctx: &mut ::milim_2d::EngineContext, parent_base: &::milim_2d::Base) {
                match self{
                    #(#match_arms_draw)*
                }
            }
            fn dispatch_destroy(&mut self, ctx: &mut ::milim_2d::EngineContext) {
                match self{
                    #(#match_arms_destroy)*
                }
            }
        }
    }.into()
}