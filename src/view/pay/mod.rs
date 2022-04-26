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

pub(self) mod beneficiary_row;
mod component;
mod view_model;
mod widget;

pub use self::component::Component;
pub(self) use view_model::ViewModel;
pub(self) use widget::Widgets;

use gtk::ResponseType;
use relm::StreamHandle;

use crate::view::wallet;

#[derive(Msg)]
pub enum Msg {
    Show,
    BeneficiaryChange,
    Response(ResponseType),
    SetWallet(StreamHandle<wallet::Msg>),
}
