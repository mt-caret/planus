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
pub struct Enum {}

#[derive(Clone, Debug)]
pub struct EnumVariant {}

#[derive(Clone, Debug)]
pub struct Union {}

#[derive(Clone, Debug)]
pub struct UnionVariant {}

#[derive(Clone, Debug)]
pub struct RpcService {}

#[derive(Clone, Debug)]
pub struct RpcMethod {}

fn reserve_module_name(path: &str, namespace_names: &mut NamespaceNames<'_, '_>) -> String {
    let name = path.to_snake_case().into();
    namespace_names
        .namespace_names
        .try_reserve_repeat("modules", name, '_')
        .into()
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
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

    const KEYWORDS: &'static [&'static str] = &[];

    fn generate_namespace(
        &mut self,
        namespace_names: &mut NamespaceNames<'_, '_>,
        namespace_name: &AbsolutePath,
        _namespace: &intermediate::Namespace,
    ) -> Namespace {
        let name = namespace_name.0.last().map_or_else(String::new, |name| {
            reserve_module_name(name, namespace_names)
        });
        Namespace {
            name: capitalize(&name),
        }
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
        _declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        _decl_name: &AbsolutePath,
        _decl: &intermediate::Enum,
    ) -> Enum {
        Enum {}
    }

    fn generate_union(
        &mut self,
        _declaration_names: &mut DeclarationNames<'_, '_>,
        _translated_namespaces: &[Self::NamespaceInfo],
        _decl_id: DeclarationIndex,
        _decl_name: &AbsolutePath,
        _decl: &intermediate::Union,
    ) -> Union {
        Union {}
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
        _translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::EnumInfo,
        _parent: &intermediate::Enum,
        _key: &str,
        _value: &intermediate::IntegerLiteral,
    ) -> EnumVariant {
        EnumVariant {}
    }

    fn generate_union_variant(
        &mut self,
        _translation_context: &mut DeclarationTranslationContext<'_, '_, Self>,
        _parent_info: &Self::UnionInfo,
        _parent: &intermediate::Union,
        _key: &str,
        _index: u8,
        _value: &intermediate::UnionVariant,
        _resolved_type: ResolvedType<'_, Self>,
    ) -> UnionVariant {
        UnionVariant {}
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
