use heck::ToSnakeCase;

use planus_types::intermediate::{self, AbsolutePath, DeclarationIndex};

use super::backend::{
    Backend, DeclarationNames, DeclarationTranslationContext, NamespaceNames, ResolvedType,
};

#[derive(Debug, Clone)]
pub struct OCamlBackend {}

#[derive(Clone, Debug)]
pub struct Namespace {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Table {}

#[derive(Clone, Debug)]
pub struct TableField {}

#[derive(Clone, Debug)]
pub struct Struct {}

#[derive(Clone, Debug)]
pub struct StructField {}

#[derive(Clone, Debug)]
pub struct Enum {
    pub type_name: String,
}

#[derive(Clone, Debug)]
pub struct EnumVariant {
    pub tag_name: String,
}

#[derive(Clone, Debug)]
pub struct Union {
    pub type_name: String,
}

#[derive(Clone, Debug)]
pub struct UnionVariant {
    pub tag_name: String,
}

#[derive(Clone, Debug)]
pub struct RpcService {}

#[derive(Clone, Debug)]
pub struct RpcMethod {}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn reserve_module_name(path: &str, namespace_names: &mut NamespaceNames<'_, '_>) -> String {
    let name = capitalize(&path.to_snake_case()).into();
    namespace_names
        .namespace_names
        .try_reserve_repeat("modules", name, '_')
        .into()
}

fn reserve_type_name(path: &str, declaration_names: &mut DeclarationNames<'_, '_>) -> String {
    let name = path.to_snake_case().into();
    declaration_names
        .declaration_names
        .try_reserve_repeat("types", name, '_')
        .into()
}

fn reserve_rust_enum_variant_name(
    path: &str,
    binding_kind: &'static str,
    declaration_names: &mut DeclarationNames<'_, '_>,
) -> String {
    let name = capitalize(&path.to_snake_case()).into();
    declaration_names
        .declaration_names
        .try_reserve_repeat(binding_kind, name, '_')
        .into()
}

impl Backend for OCamlBackend {
    type NamespaceInfo = Namespace;
    type TableInfo = Table;
    type TableFieldInfo = TableField;
    type StructInfo = Struct;
    type StructFieldInfo = StructField;
    type EnumInfo = Enum;
    type EnumVariantInfo = EnumVariant;
    type UnionInfo = Union;
    type UnionVariantInfo = UnionVariant;
    type RpcServiceInfo = RpcService;
    type RpcMethodInfo = RpcMethod;

    const KEYWORDS: &'static [&'static str] = &["type"];

    fn generate_namespace(
        &mut self,
        namespace_names: &mut NamespaceNames<'_, '_>,
        namespace_name: &AbsolutePath,
        _namespace: &intermediate::Namespace,
    ) -> Namespace {
        let name = namespace_name.0.last().map_or_else(String::new, |name| {
            reserve_module_name(name, namespace_names)
        });
        Namespace { name }
    }

    fn generate_table(
        &mut self,
        _declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        _decl_name: &AbsolutePath,
        _decl: &intermediate::Table,
    ) -> Table {
        Table {}
    }

    fn generate_struct(
        &mut self,
        _declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        _decl_name: &AbsolutePath,
        _decl: &intermediate::Struct,
    ) -> Struct {
        Struct {}
    }

    fn generate_enum(
        &mut self,
        declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        decl_name: &AbsolutePath,
        _decl: &intermediate::Enum,
    ) -> Enum {
        let decl_name = decl_name.0.last().unwrap();
        Enum {
            type_name: reserve_type_name(&decl_name, declaration_names),
        }
    }

    fn generate_union(
        &mut self,
        declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        decl_name: &AbsolutePath,
        _decl: &intermediate::Union,
    ) -> Union {
        let decl_name = decl_name.0.last().unwrap();
        Union {
            type_name: reserve_type_name(&decl_name, declaration_names),
        }
    }

    fn generate_rpc_service(
        &mut self,
        _declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        _decl_name: &AbsolutePath,
        _decl: &intermediate::RpcService,
    ) -> RpcService {
        RpcService {}
    }

    fn generate_table_field(
        &mut self,
        _translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::TableInfo,
        _parent: &intermediate::Table,
        _field_name: &str,
        _field: &intermediate::TableField,
        _resolved_type: ResolvedType<'_, Self>,
    ) -> TableField {
        TableField {}
    }

    fn generate_struct_field(
        &mut self,
        _translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::StructInfo,
        _parent: &intermediate::Struct,
        _field_name: &str,
        _field: &intermediate::StructField,
        _resolved_type: ResolvedType<'_, Self>,
    ) -> StructField {
        StructField {}
    }

    fn generate_enum_variant(
        &mut self,
        translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::EnumInfo,
        _parent: &intermediate::Enum,
        key: &str,
        _value: &intermediate::IntegerLiteral,
    ) -> EnumVariant {
        let tag_name = reserve_rust_enum_variant_name(
            key,
            "enum_name",
            &mut translation_context.declaration_names,
        );
        EnumVariant { tag_name }
    }

    fn generate_union_variant(
        &mut self,
        translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::UnionInfo,
        _parent: &intermediate::Union,
        key: &str,
        _index: u8,
        _value: &intermediate::UnionVariant,
        _resolved_type: ResolvedType<'_, Self>,
    ) -> UnionVariant {
        let tag_name = reserve_rust_enum_variant_name(
            key,
            "variant_name",
            &mut translation_context.declaration_names,
        );
        UnionVariant { tag_name }
    }

    fn generate_rpc_method(
        &mut self,
        _translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::RpcServiceInfo,
        _parent: &intermediate::RpcService,
        _method_name: &str,
        _method: &intermediate::RpcMethod,
    ) -> RpcMethod {
        todo!()
    }
}
