use crate::{
    api::ManagedTypeApi,
    types::{BigUint, MoaxOrDctTokenIdentifier},
};

use dharitri_codec::{
    dharitri_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode},
    CodecFrom, CodecFromSelf,
};

use crate as dharitri_wasm; // needed by the TypeAbi generated code
use crate::derive::TypeAbi;

use super::DctTokenPayment;

#[derive(
    TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi, Clone, PartialEq, Eq, Debug,
)]
pub struct MoaxOrDctTokenPayment<M: ManagedTypeApi> {
    pub token_identifier: MoaxOrDctTokenIdentifier<M>,
    pub token_nonce: u64,
    pub amount: BigUint<M>,
}

impl<M: ManagedTypeApi> MoaxOrDctTokenPayment<M> {
    pub fn no_payment() -> Self {
        MoaxOrDctTokenPayment {
            token_identifier: MoaxOrDctTokenIdentifier::moax(),
            token_nonce: 0,
            amount: BigUint::zero(),
        }
    }

    pub fn new(
        token_identifier: MoaxOrDctTokenIdentifier<M>,
        token_nonce: u64,
        amount: BigUint<M>,
    ) -> Self {
        MoaxOrDctTokenPayment {
            token_identifier,
            token_nonce,
            amount,
        }
    }

    pub fn into_tuple(self) -> (MoaxOrDctTokenIdentifier<M>, u64, BigUint<M>) {
        (self.token_identifier, self.token_nonce, self.amount)
    }
}

impl<M: ManagedTypeApi> From<DctTokenPayment<M>> for MoaxOrDctTokenPayment<M> {
    fn from(dct_payment: DctTokenPayment<M>) -> Self {
        MoaxOrDctTokenPayment {
            token_identifier: MoaxOrDctTokenIdentifier::dct(dct_payment.token_identifier),
            token_nonce: dct_payment.token_nonce,
            amount: dct_payment.amount,
        }
    }
}

impl<M> CodecFromSelf for MoaxOrDctTokenPayment<M> where M: ManagedTypeApi {}

impl<M> CodecFrom<&[u8]> for MoaxOrDctTokenPayment<M> where M: ManagedTypeApi {}
