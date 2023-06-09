use std::fmt;

use boulton_lang_types::{FragmentDirectiveUsage, ResolverDeclaration};
use common_lang_types::{
    BoultonDirectiveName, DefinedField, FieldDefinitionName, ObjectId, TypeId, TypeWithFieldsId,
    TypeWithFieldsName, UnvalidatedTypeName, WithSpan,
};
use intern::string_key::Intern;
use lazy_static::lazy_static;
use thiserror::Error;

use crate::{SchemaField, SchemaResolverDefinitionInfo, UnvalidatedSchema};

impl UnvalidatedSchema {
    pub fn process_resolver_declaration(
        &mut self,
        resolver_declaration: WithSpan<ResolverDeclaration>,
    ) -> ProcessResolverDeclarationResult<()> {
        let parent_type_id = self
            .schema_data
            .defined_types
            .get(&resolver_declaration.item.parent_type.item.into())
            .ok_or(ProcessResolverDeclarationError::MissingParent {
                parent_type_name: resolver_declaration.item.parent_type.item,
            })?;

        match parent_type_id {
            TypeId::Object(object_id) => {
                self.add_resolver_field_to_object(*object_id, resolver_declaration)?;
            }
            TypeId::Scalar(scalar_id) => {
                let scalar_name = self.schema_data.scalars[scalar_id.as_usize()].name;
                return Err(ProcessResolverDeclarationError::InvalidParentType {
                    parent_type: "scalar",
                    parent_type_name: scalar_name.into(),
                });
            }
        }

        Ok(())
    }

    fn add_resolver_field_to_object(
        &mut self,
        object: ObjectId,
        resolver_declaration: WithSpan<ResolverDeclaration>,
    ) -> ProcessResolverDeclarationResult<()> {
        let object = &mut self.schema_data.objects[object.as_usize()];
        let resolver_field_name = resolver_declaration.item.resolver_field_name.item;

        if object
            .encountered_field_names
            .insert(
                resolver_field_name.into(),
                DefinedField::ResolverField(resolver_field_name),
            )
            .is_some()
        {
            // Did not insert, so this object already has a field with the same name :(
            return Err(ProcessResolverDeclarationError::ParentAlreadyHasField {
                parent_type: "object",
                parent_type_name: object.name.into(),
                resolver_field_name: resolver_field_name.into(),
            });
        }

        let next_field_id = self.fields.len().into();
        object.fields.push(next_field_id);

        let name = resolver_declaration.item.resolver_field_name.item.into();
        let variant = get_resolver_variant(&resolver_declaration.item.directives);
        let has_associated_js_function = resolver_declaration.item.has_associated_js_function;

        // TODO variant should carry payloads, instead of this check
        if variant.as_ref().map(|span| span.item) == Some(ResolverVariant::Component) {
            if !has_associated_js_function {
                return Err(ProcessResolverDeclarationError::ComponentResolverMissingJsFunction {});
            }
        }

        self.fields.push(SchemaField {
            description: resolver_declaration.item.description.map(|d| d.item),
            name,
            id: next_field_id,
            field_type: DefinedField::ResolverField(SchemaResolverDefinitionInfo {
                resolver_definition_path: resolver_declaration.item.resolver_definition_path,
                selection_set_and_unwraps: resolver_declaration.item.selection_set_and_unwraps,
                field_id: next_field_id,
                variant,
                is_fetchable: is_fetchable(&resolver_declaration.item.directives),
                variable_definitions: resolver_declaration.item.variable_definitions,
                type_and_field: format!("{}__{}", object.name, name).intern().into(),
                has_associated_js_function,
            }),
            parent_type_id: TypeWithFieldsId::Object(object.id),
        });
        Ok(())
    }
}

type ProcessResolverDeclarationResult<T> = Result<T, ProcessResolverDeclarationError>;

#[derive(Error, Debug)]
pub enum ProcessResolverDeclarationError {
    #[error("Missing parent type. Type: `{parent_type_name}`")]
    MissingParent {
        parent_type_name: UnvalidatedTypeName,
    },

    #[error("Invalid parent type. `{parent_type_name}` is a {parent_type}, but it should be an object or interface.")]
    InvalidParentType {
        parent_type: &'static str,
        parent_type_name: UnvalidatedTypeName,
    },

    #[error(
        "The {parent_type} {parent_type_name} already has a field named `{resolver_field_name}`."
    )]
    ParentAlreadyHasField {
        parent_type: &'static str,
        parent_type_name: TypeWithFieldsName,
        resolver_field_name: FieldDefinitionName,
    },

    #[error(
        "Resolvers with @component must have associated javascript (i.e. bDeclare`...` must be called as a function, as in bDeclare`...`(MyComponent))"
    )]
    ComponentResolverMissingJsFunction {
        // TODO add parent type and resolver field name
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolverVariant {
    Component,
    Eager,
}

impl fmt::Display for ResolverVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolverVariant::Component => write!(f, "Component"),
            ResolverVariant::Eager => write!(f, "Eager"),
        }
    }
}

lazy_static! {
    // This is regex is inadequate, as bDeclare<typeof foo`...`>, and it's certainly possible
    // to want that.
    static ref EAGER: BoultonDirectiveName = "eager".intern().into();
    static ref COMPONENT: BoultonDirectiveName = "component".intern().into();
    static ref FETCHABLE: BoultonDirectiveName = "fetchable".intern().into();
}

// TODO validate that the type is actually fetchable, and that we don't have both
fn get_resolver_variant(
    directives: &[WithSpan<FragmentDirectiveUsage>],
) -> Option<WithSpan<ResolverVariant>> {
    for directive in directives.iter() {
        let span = directive.span;
        if directive.item.name.item == *EAGER {
            return Some(WithSpan::new(ResolverVariant::Eager, span));
        } else if directive.item.name.item == *COMPONENT {
            return Some(WithSpan::new(ResolverVariant::Component, span));
        }
    }
    None
}

fn is_fetchable(directives: &[WithSpan<FragmentDirectiveUsage>]) -> bool {
    for directive in directives.iter() {
        if directive.item.name.item == *FETCHABLE {
            return true;
        }
    }
    false
}
