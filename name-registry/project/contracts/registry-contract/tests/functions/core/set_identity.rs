use crate::utils::{
    interface::core::{register, set_identity},
    setup::{setup, REGISTER_DURATION},
};
use fuels::{prelude::Address, types::Identity};

mod success {
    use super::*;
    use crate::utils::{
        interface::info::identity,
        setup::{string_to_ascii, IdentityChangedEvent},
    };

    #[tokio::test]
    async fn can_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
        )
        .await;

        let previous_identity = identity(&instance, &acc1.name).await;

        assert_eq!(previous_identity.value.unwrap(), acc1.identity(),);

        let response = set_identity(&instance, &acc1.name, wallet_identity2.clone()).await;

        let new_identity = identity(&instance, &acc1.name).await;

        assert_eq!(new_identity.value.unwrap(), wallet_identity2);

        let log = response
            .get_logs_with_type::<IdentityChangedEvent>()
            .unwrap();
        assert_eq!(
            log,
            vec![IdentityChangedEvent {
                name: string_to_ascii(&acc1.name),
                new_identity: wallet_identity2,
                previous_identity: acc1.identity(),
            }]
        )
    }
}

mod revert {
    use super::*;
    use crate::utils::setup::{setup, REGISTER_DURATION};

    // TODO: missing tests

    #[tokio::test]
    #[should_panic(expected = "SenderNotOwner")]
    async fn cant_set_identity() {
        let (instance, acc1, wallet2) = setup().await;
        let wallet_identity2 = Identity::Address(Address::from(wallet2.address()));

        register(
            &instance,
            &acc1.name,
            REGISTER_DURATION,
            &acc1.identity(),
            &acc1.identity(),
        )
        .await;

        set_identity(
            &instance.with_wallet(wallet2).unwrap(),
            &acc1.name,
            wallet_identity2.clone(),
        )
        .await;
    }
}
