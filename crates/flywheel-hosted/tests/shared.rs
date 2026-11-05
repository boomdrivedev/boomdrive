//! The hosted implementation passes `flywheel-control`'s shared cases
//! (the control-plane-interface spec's "Every implementation passes the
//! shared cases", its scenario "The hosted implementation passes"; Task
//! notes 5.3), over the fake hosted control plane on the loopback
//! address, holding the wants of one partition, `home`. The harness can
//! see the report that control plane holds.

mod common;

use common::fake::Fake;
use ed25519_dalek::SigningKey;
use flywheel_control::shared::{self, Harness, REPOSITORY};
use flywheel_control::{Id, Want};
use flywheel_hosted::key::{Memory, Store};
use flywheel_hosted::table::Table;
use flywheel_hosted::Hosted;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// `home`'s key, fixed bytes of the test's own.
fn home_key() -> SigningKey {
    SigningKey::from_bytes(&[5u8; 32])
}

/// A fake hosted control plane holding `home`'s wants.
struct Hosteds {
    fake: Fake,
}

impl Hosteds {
    fn new() -> Hosteds {
        let fake = Fake::start();
        fake.enrolled("home", &home_key().verifying_key());
        Hosteds { fake }
    }
}

/// `id` as `home`'s control plane gives it, its prefix taken off.
fn bare(id: &Id) -> Id {
    Id::new(id.as_str().trim_start_matches("home/"))
}

/// `want` as `home`'s control plane gives it, each id's prefix taken off.
fn given(want: &Want) -> Want {
    match want.clone() {
        Want::Offer(mut offer) => {
            offer.id = bare(&offer.id);
            offer.launch.after = offer.launch.after.iter().map(bare).collect();
            Want::Offer(offer)
        }
        Want::Act(mut act) => {
            act.id = bare(&act.id);
            act.launch = bare(&act.launch);
            Want::Act(act)
        }
        Want::Command(mut command) => {
            command.id = bare(&command.id);
            Want::Command(command)
        }
        Want::Inbox(mut inbox) => {
            inbox.id = bare(&inbox.id);
            Want::Inbox(inbox)
        }
    }
}

impl Harness for Hosteds {
    type Plane = Hosted;
    type Seen = Value;

    fn id(&self, n: u32) -> Id {
        Id::new(format!("home/w-{n}"))
    }

    fn plane(&mut self, wants: &[Want]) -> Hosted {
        let given: Vec<Want> = wants.iter().map(given).collect();
        self.fake.set_wants(
            "home",
            serde_json::to_value(given).expect("the wants as version 1"),
        );
        let text = format!(
            "url = {:?}\n[[partitions]]\nlabel = \"home\"\nrepositories = [{REPOSITORY:?}]\n",
            self.fake.url()
        );
        let table = Table::read(
            &toml::from_str(&text).expect("a TOML table"),
            Path::new("/h/.config/flywheel/config.toml"),
        )
        .expect("the table read");
        let store = Memory::default();
        store
            .write(&table.key_name("home"), &home_key())
            .expect("kept");
        Hosted::new(
            &table,
            Arc::new(store),
            Duration::from_secs(60),
            Box::new(|_| {}),
            Box::new(|| {}),
        )
        .expect("opened")
    }

    fn seen(&mut self, _: &Hosted) -> Value {
        self.fake.held("home")
    }
}

#[test]
fn wants_in_order() {
    shared::wants_in_order(&mut Hosteds::new());
}

#[test]
fn a_report_comes_back() {
    shared::a_report_comes_back(&mut Hosteds::new());
}

#[test]
fn a_report_twice() {
    shared::a_report_twice(&mut Hosteds::new());
}

#[test]
fn a_settled_want_is_given() {
    shared::a_settled_want_is_given(&mut Hosteds::new());
}
