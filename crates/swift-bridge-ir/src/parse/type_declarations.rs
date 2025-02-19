use crate::bridged_type::{
    BridgedType, CustomBridgedType, OpaqueForeignType, SharedEnum, SharedStruct, SharedType,
};
use crate::parse::HostLang;
use crate::SWIFT_BRIDGE_PREFIX;
use proc_macro2::Ident;
use quote::ToTokens;
use std::collections::HashMap;
use std::ops::Deref;
use syn::{GenericParam, PatType, Type, TypePath};

#[derive(Default)]
pub(crate) struct TypeDeclarations {
    decls: HashMap<String, TypeDeclaration>,
    order: Vec<String>,
}

#[derive(Clone)]
pub(crate) enum TypeDeclaration {
    Shared(SharedTypeDeclaration),
    Opaque(OpaqueForeignTypeDeclaration),
}

#[derive(Clone)]
pub(crate) enum SharedTypeDeclaration {
    Struct(SharedStruct),
    Enum(SharedEnum),
}

impl TypeDeclaration {
    pub fn to_bridged_type(&self, reference: bool, mutable: bool) -> BridgedType {
        match self {
            TypeDeclaration::Shared(SharedTypeDeclaration::Struct(shared_struct)) => {
                BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Struct(
                    SharedStruct {
                        name: shared_struct.name.clone(),
                        swift_repr: shared_struct.swift_repr,
                        fields: shared_struct.fields.clone(),
                        swift_name: shared_struct.swift_name.clone(),
                        already_declared: shared_struct.already_declared,
                    },
                )))
            }
            TypeDeclaration::Shared(SharedTypeDeclaration::Enum(shared_enum)) => {
                BridgedType::Foreign(CustomBridgedType::Shared(SharedType::Enum(SharedEnum {
                    name: shared_enum.name.clone(),
                    variants: shared_enum.variants.clone(),
                })))
            }
            TypeDeclaration::Opaque(opaque) => {
                BridgedType::Foreign(CustomBridgedType::Opaque(OpaqueForeignType {
                    ty: opaque.ty.clone(),
                    host_lang: opaque.host_lang,
                    reference,
                    mutable,
                }))
            }
        }
    }
}

#[derive(Clone)]
pub(crate) struct OpaqueForeignTypeDeclaration {
    pub ty: Ident,
    pub host_lang: HostLang,
    /// Whether or not the `#[swift_bridge(already_declared)]` attribute was present on the type.
    /// If it was, we won't generate Swift and C type declarations for this type, since we
    /// will elsewhere.
    pub already_declared: bool,
    /// A doc comment.
    // TODO: Use this to generate doc comment for the generated Swift type.
    #[allow(unused)]
    pub doc_comment: Option<String>,
    #[allow(unused)]
    pub generics: Vec<GenericParam>,
}

impl Deref for OpaqueForeignTypeDeclaration {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.ty
    }
}

impl OpaqueForeignTypeDeclaration {
    // "__swift_bridge__$TypeName$_free"
    pub fn free_link_name(&self) -> String {
        format!("{}${}$_free", SWIFT_BRIDGE_PREFIX, self.ty.to_string())
    }

    // "__swift_bridge__TypeName__free"
    pub fn free_func_name(&self) -> String {
        format!("{}{}__free", SWIFT_BRIDGE_PREFIX, self.ty.to_string())
    }

    pub fn ty_name_ident(&self) -> &Ident {
        &self.ty
    }
}

impl TypeDeclarations {
    pub(crate) fn get<Q: ?Sized>(&self, type_name: &Q) -> Option<&TypeDeclaration>
    where
        Q: std::hash::Hash + Eq,
        String: std::borrow::Borrow<Q>,
    {
        self.decls.get(type_name)
    }

    pub(crate) fn get_with_pat_type(&self, pat_ty: &PatType) -> Option<&TypeDeclaration> {
        self.get_with_type(&pat_ty.ty)
    }

    pub(crate) fn get_with_type_path(&self, type_path: &TypePath) -> Option<&TypeDeclaration> {
        let ty = type_path.path.to_token_stream().to_string();
        self.get(&ty)
    }

    pub(crate) fn get_with_type(&self, ty: &Type) -> Option<&TypeDeclaration> {
        let ty = match ty.deref() {
            Type::Reference(reference) => reference.elem.to_token_stream().to_string(),
            Type::Path(path) => path.to_token_stream().to_string(),
            _ => todo!("Handle other cases"),
        };
        self.get(&ty)
    }

    pub(crate) fn insert(&mut self, type_name: String, ty: TypeDeclaration) {
        self.decls.insert(type_name.clone(), ty);
        self.order.push(type_name);
    }

    pub fn types(&self) -> Vec<&TypeDeclaration> {
        self.order
            .iter()
            .map(|ty| self.decls.get(ty).unwrap())
            .collect()
    }
}

#[cfg(test)]
impl TypeDeclaration {
    pub fn unwrap_shared_struct(&self) -> &SharedStruct {
        match self {
            TypeDeclaration::Shared(SharedTypeDeclaration::Struct(s)) => s,
            _ => panic!(),
        }
    }

    pub fn unwrap_shared_enum(&self) -> &SharedEnum {
        match self {
            TypeDeclaration::Shared(SharedTypeDeclaration::Enum(e)) => e,
            _ => panic!(),
        }
    }

    pub fn unwrap_opaque(&self) -> &OpaqueForeignTypeDeclaration {
        match self {
            TypeDeclaration::Opaque(o) => o,
            _ => panic!(),
        }
    }
}
