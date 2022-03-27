#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::prelude::*;
use crate::{
	cod::{
		components::{
			aspirations::Aspirations,
			experience::Experience,
			merits::Merits,
			traits::{
				Attributes,
				Skills,
				SkillSpecialties,
			}
		},
		vtr2e::{
			advantages::bloodPotencyAttributeMax,
			components::{
				advantages::Advantages,
				details::Details,
				disciplines::{
					Disciplines,
					Devotions,
				},
				touchstones::Touchstones,
			},
			state::KindredAdvantages,
		},
	},
};

pub fn VampireSheet(cx: Scope) -> Element
{
	let advantages = use_atom_ref(&cx, KindredAdvantages);
	let bloodPotency = advantages.read().bloodPotency;
	let traitMax = bloodPotencyAttributeMax(bloodPotency);
	
	return cx.render(rsx!
	{	
		div
		{
			class: "sheet cod column",
			
			h1 { "Vampire: The Requiem" }
			h3 { "Second Edition" }
			hr { class: "row" }
			div { class: "row", Details {} Advantages {} }
			hr { class: "row" }
			div { class: "row spacedOut", Aspirations {} Touchstones {} Experience {} }
			hr { class: "row" }
			div { class: "row", Attributes { traitMax: traitMax } }
			hr { class: "row" }
			div { class: "row", Skills { traitMax: traitMax } }
			hr { class: "row" }
			div { class: "row", Disciplines {} SkillSpecialties {} Merits {} }
			hr { class: "row" }
			div { class: "row", Devotions {} }
		}
	});
}
