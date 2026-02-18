//! Integration tests for reusability builder helpers.

use aps_namespaced_ownables::{
    SetNamespacedOwnableNamespaceId, TrySetNamespacedOwnableDescription,
    TrySetNamespacedOwnableName,
};
use aps_namespaces::namespaces;
use aps_ownables::{SetOwnableCreatorId, SetOwnableEditorId, SetOwnableOwnerId};
use aps_physical_asset_models::physical_asset_models;
use aps_pipette_tip_models::pipette_tip_models;
use aps_test_utils::{aps_conn, namespace, user};
use aps_traits::ReusabilityBuilderExt;
use diesel::associations::Identifiable;
use diesel_builders::{prelude::*, set_column::TrySetColumnExt};

#[test]
fn physical_asset_model_builder_supports_reusability_helpers() {
    let mut conn = aps_conn();
    let test_user = user(&mut conn);
    let test_namespace = namespace("aps-traits-reusability", &test_user, &mut conn);

    let single_use = physical_asset_models::table::builder()
        .try_name("Single Use Physical Model")
        .expect("Failed to set model name")
        .try_description("Physical model expected to be used only once.")
        .expect("Failed to set model description")
        .try_single_use()
        .expect("Failed to mark model as single-use")
        .creator_id(*test_user.id())
        .editor_id(*test_user.id())
        .owner_id(*test_user.id())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(&mut conn)
        .expect("Failed to insert single-use physical asset model");

    assert_eq!(single_use.get_column::<physical_asset_models::lifecycle_class_id>(), "single_use");
    assert_eq!(single_use.get_column::<physical_asset_models::recommended_max_use>(), None);

    let reusable = physical_asset_models::table::builder()
        .try_name("Reusable Physical Model")
        .expect("Failed to set model name")
        .try_description("Physical model expected to be reused up to ten times.")
        .expect("Failed to set model description")
        .try_reusable(10)
        .expect("Failed to mark model as reusable")
        .creator_id(*test_user.id())
        .editor_id(*test_user.id())
        .owner_id(*test_user.id())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(&mut conn)
        .expect("Failed to insert reusable physical asset model");

    assert_eq!(reusable.get_column::<physical_asset_models::lifecycle_class_id>(), "reusable");
    assert_eq!(reusable.get_column::<physical_asset_models::recommended_max_use>(), Some(10));
}

#[test]
fn descendant_builder_supports_reusability_helpers() {
    let mut conn = aps_conn();
    let test_user = user(&mut conn);
    let test_namespace = namespace("aps-traits-reusability-desc", &test_user, &mut conn);

    let pipette_tip_model = pipette_tip_models::table::builder()
        .try_name("Single Use Pipette Tip Model")
        .expect("Failed to set pipette-tip model name")
        .try_description("Pipette tip model expected to be single-use.")
        .expect("Failed to set pipette-tip model description")
        .try_single_use()
        .expect("Failed to mark pipette-tip model as single-use")
        .creator_id(*test_user.id())
        .editor_id(*test_user.id())
        .owner_id(*test_user.id())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(&mut conn)
        .expect("Failed to insert pipette-tip model");

    assert_eq!(
        pipette_tip_model.get_column::<physical_asset_models::lifecycle_class_id>(),
        "single_use"
    );
    assert_eq!(pipette_tip_model.get_column::<physical_asset_models::recommended_max_use>(), None);
}

#[test]
#[ignore = "pg2sqlite test backend removes cross-column CHECK constraints"]
fn rejects_non_reusable_models_with_recommended_max_use() {
    let mut conn = aps_conn();
    let test_user = user(&mut conn);
    let test_namespace = namespace("aps-traits-reusability-invalid", &test_user, &mut conn);

    let result = physical_asset_models::table::builder()
        .try_name("Invalid Non-Reusable Model")
        .expect("Failed to set model name")
        .try_description("Invalid non-reusable model with recommended max use.")
        .expect("Failed to set model description")
        .try_set_column::<physical_asset_models::lifecycle_class_id>("single_use")
        .expect("Failed to set lifecycle class")
        .try_set_column::<physical_asset_models::recommended_max_use>(Some(5_i16))
        .expect("Failed to set recommended max use")
        .creator_id(*test_user.id())
        .editor_id(*test_user.id())
        .owner_id(*test_user.id())
        .namespace_id(test_namespace.get_column::<namespaces::id>())
        .insert_nested(&mut conn);

    assert!(result.is_err());
}

#[test]
fn rejects_non_positive_reusable_max_uses() {
    let mut conn = aps_conn();
    let test_user = user(&mut conn);
    let test_namespace = namespace("aps-traits-reusability-max", &test_user, &mut conn);

    let result = physical_asset_models::table::builder()
        .try_name("Invalid Reusable Model")
        .expect("Failed to set model name")
        .try_description("Invalid reusable model with non-positive max uses.")
        .expect("Failed to set model description")
        .try_reusable(0)
        .map(|builder| {
            builder
                .creator_id(*test_user.id())
                .editor_id(*test_user.id())
                .owner_id(*test_user.id())
                .namespace_id(test_namespace.get_column::<namespaces::id>())
        });

    assert!(result.is_err());
}
