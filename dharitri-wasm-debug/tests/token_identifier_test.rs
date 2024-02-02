use dharitri_wasm::types::{
    BoxedBytes, MoaxOrDctTokenIdentifier, MoaxOrDctTokenPayment, DctTokenPayment,
    TokenIdentifier,
};
use dharitri_wasm_debug::{
    check_managed_top_encode_decode, managed_moax_token_id, managed_token_id,
    managed_token_id_wrapped, DebugApi,
};

#[test]
fn test_moax() {
    let _ = DebugApi::dummy();
    assert!(MoaxOrDctTokenIdentifier::<DebugApi>::moax().is_moax());
}

#[test]
fn test_codec() {
    let api = DebugApi::dummy();
    check_managed_top_encode_decode(
        api.clone(),
        MoaxOrDctTokenIdentifier::<DebugApi>::moax(),
        MoaxOrDctTokenIdentifier::<DebugApi>::MOAX_REPRESENTATION,
    );

    let expected = BoxedBytes::from_concat(&[
        &[0, 0, 0, 4],
        &MoaxOrDctTokenIdentifier::<DebugApi>::MOAX_REPRESENTATION[..],
    ]);
    check_managed_top_encode_decode(
        api.clone(),
        vec![MoaxOrDctTokenIdentifier::<DebugApi>::moax()],
        expected.as_slice(),
    );
}

#[test]
#[rustfmt::skip]
fn test_is_valid_dct_identifier() {
    let _ = DebugApi::dummy();

    // valid identifier
    assert!(TokenIdentifier::<DebugApi>::from("ALC-6258d2").is_valid_dct_identifier());

    // valid identifier with numbers in ticker
    assert!(TokenIdentifier::<DebugApi>::from("ALC123-6258d2").is_valid_dct_identifier());

    // valid ticker only numbers
    assert!(TokenIdentifier::<DebugApi>::from("12345-6258d2").is_valid_dct_identifier());

    // missing dash
    assert!(!TokenIdentifier::<DebugApi>::from("ALC6258d2").is_valid_dct_identifier());

    // wrong dash position
    assert!(!TokenIdentifier::<DebugApi>::from("AL-C6258d2").is_valid_dct_identifier());

    // lowercase ticker
    assert!(!TokenIdentifier::<DebugApi>::from("alc-6258d2").is_valid_dct_identifier());

    // uppercase random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258D2").is_valid_dct_identifier());

    // too many random chars
    assert!(!TokenIdentifier::<DebugApi>::from("ALC-6258d2ff").is_valid_dct_identifier());

    // ticker too short
    assert!(!TokenIdentifier::<DebugApi>::from("AL-6258d2").is_valid_dct_identifier());

    // ticker too long
    assert!(!TokenIdentifier::<DebugApi>::from("ALCCCCCCCCC-6258d2").is_valid_dct_identifier());
}

#[test]
fn test_is_valid_moax_or_dct() {
    let _ = DebugApi::dummy();

    // moax is always valid
    assert!(MoaxOrDctTokenIdentifier::<DebugApi>::moax().is_valid());

    // valid dct
    assert!(
        MoaxOrDctTokenIdentifier::<DebugApi>::dct(TokenIdentifier::from("ALC-6258d2")).is_valid()
    );

    // invalid dct, see above
    assert!(
        !MoaxOrDctTokenIdentifier::<DebugApi>::dct(TokenIdentifier::from("ALCCCCCCCCC-6258d2"))
            .is_valid()
    );
}

#[test]
fn test_token_identifier_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        TokenIdentifier::<DebugApi>::from("DCT-00000"),
        TokenIdentifier::<DebugApi>::from("DCT-00000")
    );
    assert_ne!(
        TokenIdentifier::<DebugApi>::from("DCT-00001"),
        TokenIdentifier::<DebugApi>::from("DCT-00002")
    );

    assert_eq!(
        MoaxOrDctTokenIdentifier::<DebugApi>::dct(TokenIdentifier::from("DCT-00003")),
        TokenIdentifier::<DebugApi>::from("DCT-00003")
    );
    assert_ne!(
        MoaxOrDctTokenIdentifier::<DebugApi>::moax(),
        TokenIdentifier::<DebugApi>::from("ANYTHING-1234")
    );
    assert_ne!(
        MoaxOrDctTokenIdentifier::<DebugApi>::moax(),
        TokenIdentifier::<DebugApi>::from("MOAX")
    );
}

#[test]
fn test_payment_eq() {
    let _ = DebugApi::dummy();
    assert_eq!(
        DctTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
        DctTokenPayment::<DebugApi>::new("PAY-00000".into(), 0, 1000u32.into()),
    );
    assert_ne!(
        DctTokenPayment::<DebugApi>::new("PAY-00001".into(), 0, 1000u32.into()),
        DctTokenPayment::<DebugApi>::new("PAY-00002".into(), 0, 1000u32.into()),
    );
    assert_eq!(
        MoaxOrDctTokenPayment::<DebugApi>::no_payment(),
        MoaxOrDctTokenPayment::<DebugApi>::no_payment(),
    );
    assert_eq!(
        MoaxOrDctTokenPayment::<DebugApi>::new(
            MoaxOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
        MoaxOrDctTokenPayment::<DebugApi>::new(
            MoaxOrDctTokenIdentifier::dct("DCTPAY-00000"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        MoaxOrDctTokenPayment::<DebugApi>::new(
            MoaxOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        MoaxOrDctTokenPayment::<DebugApi>::new(
            MoaxOrDctTokenIdentifier::dct("DCTPAY-00002"),
            0,
            1000u32.into()
        ),
    );
    assert_ne!(
        MoaxOrDctTokenPayment::<DebugApi>::new(
            MoaxOrDctTokenIdentifier::dct("DCTPAY-00001"),
            0,
            1000u32.into()
        ),
        MoaxOrDctTokenPayment::<DebugApi>::no_payment(),
    );
}

#[test]
fn test_managed_token_id_macro() {
    let _ = DebugApi::dummy();
    assert_eq!(
        managed_moax_token_id!(),
        MoaxOrDctTokenIdentifier::<DebugApi>::moax()
    );
    assert_eq!(
        managed_token_id!(b"ALC-6258d2"),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    );
    assert_eq!(
        managed_token_id_wrapped!(b"ALC-6258d2").unwrap_dct(),
        TokenIdentifier::<DebugApi>::from("ALC-6258d2")
    )
}
