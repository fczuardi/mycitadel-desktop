// MyCitadel desktop wallet: bitcoin & RGB wallet based on GTK framework.
//
// Written in 2022 by
//     Dr. Maxim Orlovsky <orlovsky@pandoraprime.ch>
//
// Copyright (C) 2022 by Pandora Prime Sarl, Switzerland.
//
// This software is distributed without any warranty. You should have received
// a copy of the AGPL-3.0 License along with this software. If not, see
// <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

use std::collections::BTreeSet;
use wallet::descriptors::DescrVariants;
use wallet::psbt::Psbt;

use crate::model::{PublicNetwork, Signer, SpendingCondition};

// TODO: Move to citadel-runtime
#[derive(Getters, Clone, Eq, PartialEq, Debug, Default)]
pub struct Wallet {
    #[getter(skip)]
    descriptor: WalletDescriptor,
    state: WalletState,
    history: Vec<Psbt>,
    wip: Vec<Psbt>,
}

impl Wallet {
    pub fn with(descriptor: WalletDescriptor) -> Self {
        Wallet {
            descriptor,
            ..default!()
        }
    }

    pub fn as_descriptor(&self) -> &WalletDescriptor {
        &self.descriptor
    }

    pub fn to_descriptor(&self) -> WalletDescriptor {
        self.descriptor.clone()
    }

    pub fn set_descriptor(&mut self, descr: WalletDescriptor) {
        self.state = WalletState::default();
        self.history.clear();
        self.wip.clear();
        self.descriptor = descr;
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct WalletDescriptor {
    format: WalletFormat,
    signers: BTreeSet<Signer>,
    conditions: Vec<SpendingCondition>,
    network: PublicNetwork,
}

// TODO: Move to descriptor wallet library
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum WalletFormat {
    LnpBp(DescrVariants),
    Bip43(Bip43Format),
}

impl Default for WalletFormat {
    fn default() -> Self {
        WalletFormat::Bip43(Bip43::Bip48Native)
    }
}

/// BIP43-based purpose fields and derivation paths formats defined by them.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
// TODO: Move to descriptor wallet library
pub enum Bip43 {
    /// Account-based P2PKH derivation
    ///
    /// `m / 44' / coin_type' / account'`
    #[display("bip44", alt = "m/44h")]
    Bip44,

    /// Cosigner-index-based multisig derivation
    ///
    /// `m / 45' / cosigner_index`
    #[display("bip45", alt = "m/45h")]
    Bip45,

    /// Account-based multisig derivation with sorted keys & P2WSH nested scripts
    ///
    /// `m / 48' / coin_type' / account' / script_type'`
    #[display("bip48-nested", alt = "m/48h//1h")]
    Bip48Nested,

    /// Account-based multisig derivation with sorted keys & P2WSH native scripts
    ///
    /// `m / 48' / coin_type' / account' / script_type'`
    #[display("bip48-native", alt = "m/48h//2h")]
    Bip48Native,

    /// Account-based legacy P2WPKH-in-P2SH derivation
    ///
    /// `m / 49' / coin_type' / account'`
    #[display("bip49", alt = "m/49h")]
    Bip49,

    /// Account-based native P2WPKH derivation
    ///
    /// `m / 84' / coin_type' / account'`
    #[display("bip84", alt = "m/84h")]
    Bip84,

    /// Account-based single-key P2TR derivation
    ///
    /// `m / 86' / coin_type' / account'`
    #[display("bip86", alt = "m/86h")]
    Bip86,

    /// Account- & descriptor-based derivation for multi-sig wallets
    #[display("bip87", alt = "m/87h")]
    ///
    /// `m / 87' / coin_type' / account'`
    Bip87,
}

impl Bip43 {
    pub fn singlesig_pkh() -> Bip43 {
        Bip43::Bip44
    }
    pub fn singlesig_nested0() -> Bip43 {
        Bip43::Bip49
    }
    pub fn singlesig_segwit0() -> Bip43 {
        Bip43::Bip84
    }
    pub fn singlelsig_taproot() -> Bip43 {
        Bip43::Bip86
    }
    pub fn multisig_ordered_sh() -> Bip43 {
        Bip43::Bip45
    }
    pub fn multisig_nested0() -> Bip43 {
        Bip43::Bip48Nested
    }
    pub fn multisig_segwit0() -> Bip43 {
        Bip43::Bip48Native
    }
    pub fn multisig_descriptor() -> Bip43 {
        Bip43::Bip87
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct WalletState {
    balance: Sats,
}

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
pub struct Sats(u64);
