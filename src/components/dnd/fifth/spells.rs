#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::{
	events::FormEvent,
	prelude::*,
};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use crate::{
	components::core::{
		check::{
			CheckCircle,
			CheckLine,
			CheckLineState,
			getSingleCheckLineState,
		},
		events::{
			hideRemovePopUp,
			showRemovePopUpWithIndex,
		},
		tracks::Track,
	},
	core::{
		structs::Tracker,
		util::{
			RemovePopUpXOffset,
			RemovePopUpYOffset,
		},
	},
	dnd::fifth::{
		enums::{
			CasterWeight,
			MagicSchool,
		},
		state::{
			AdventurerClassLevels,
		},
		structs::{
			ClassLevel,
			Spell,
		},
		util::{
			calculateCasterLevel,
			getSpellSlots,
		},
	},
};

#[derive(PartialEq, Props)]
pub struct EditSpellProps
{
	pub spell: Spell,
	pub spellIndex: usize,
}

pub fn EditSpell(cx: Scope<EditSpellProps>) -> Element
{
	let concentration = getSingleCheckLineState(cx.props.spell.concentration);
	let verbal = getSingleCheckLineState(cx.props.spell.components.verbal);
	let somatic = getSingleCheckLineState(cx.props.spell.components.somatic);
	let material = getSingleCheckLineState(cx.props.spell.components.material);
	
	let mut magicSchools = vec![];
	for school in MagicSchool::iter()
	{
		magicSchools.push(school);
	}
	
	let castingTime = match &cx.props.spell.castingTime
	{
		Some(value) => value.clone(),
		None => "".to_string(),
	};
	
	let duration = match &cx.props.spell.duration
	{
		Some(value) => value.clone(),
		None => "".to_string(),
	};
	
	let materials = match &cx.props.spell.materials
	{
		Some(value) => value.clone(),
		None => "".to_string(),
	};
	
	let range = match &cx.props.spell.range
	{
		Some(value) => value.clone(),
		None => "".to_string(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "column justStart editSpell",
			
			div
			{
				class: "row justEven",
				
				div { class: "label colOne", "Name:" }
				input { class: "grow", r#type: "text", value: "{cx.props.spell.name}" }
				
				div { class: "label colTwo", "Level:" }
				select
				{
					option { value: "", "Choose Level" }
					(0..=9).map(|i|
					{
						let label = match i == 0
						{
							true => "Cantrip".to_string(),
							false => i.to_string(),
						};
						let selected = match cx.props.spell.level == i
						{
							true => "true",
							false => "false",
						};
						rsx!(option { key: "{i}", value: "{i}", selected: "{selected}", "{label}" })
					})
				}
				
				div { class: "label colThree", "School:" }
				select
				{
					option { value: "", "Choose School" }
					magicSchools.iter().enumerate().map(|(i, school)|
					{
						let selected = match &cx.props.spell.school == school
						{
							true => "true",
							false => "false",
						};
						rsx!(option { key: "{i}", value: "{school}", selected: "{selected}", "{school}" })
					})
				}
			}
			
			div
			{
				class: "row justEven",
				
				div { class: "label colOne", "Casting Time:" }
				input { class: "grow", r#type: "text", value: "{castingTime}" }
				
				div { class: "label colTwo", "Range:" }
				input { class: "grow", r#type: "text", value: "{range}" }
				
				div { class: "label colThree", "Duration:" }
				input { class: "grow", r#type: "text", value: "{duration}" }
			}
			
			div
			{
				class: "row justEven checkerRow",
				
				div
				{
					class: "row justAround",
					
					div { class: "label", "Concentration" }
					CheckLine { lineState: concentration }
				}
				
				div
				{
					class: "row justAround",
					
					div { class: "label", "Verbal" }
					CheckLine { lineState: verbal }
				}
				
				div
				{
					class: "row justAround",
					
					div { class: "label", "Somatic" }
					CheckLine { lineState: somatic }
				}
				
				div
				{
					class: "row justAround",
					
					div { class: "label", "Material" }
					CheckLine { lineState: material }
				}
			}
			
			cx.props.spell.components.material
				.then(|| rsx!(div
				{
					class: "row justEven",
					input { class: "materialComponents", placeholder: "Required materials", title: "Required materials", value: "{materials}" }
				}))
			
			div
			{
				class: "column justStart",
				
				div
				{
					class: "row justStart",
					
					div { class: "label description", "Description:" }
				}
				
				textarea { "{cx.props.spell.description}" }
			}
		}
	});
}

// --------------------------------------------------

#[derive(PartialEq, Props)]
pub struct KnownSpellsProps
{
	pub spells: Vec<Spell>,
}

pub fn KnownSpells(cx: Scope<KnownSpellsProps>) -> Element
{
	let clickedX = use_state(&cx, || 0);
	let clickedY = use_state(&cx, || 0);
	let lastIndex = use_state(&cx, || 0);
	let showRemove = use_state(&cx, || false);
	
	let posX = *clickedX.get() - RemovePopUpXOffset;
	let posY = *clickedY.get() - RemovePopUpYOffset;
	
	return cx.render(rsx!
	{
		div
		{
			class: "column justEven knownSpells",
			
			h3 { class: "row justEven", "Known Spells" }
			
			div
			{
				class: "column justEven",
				
				cx.props.spells.iter().enumerate().map(|(i, spell)|
				{
					rsx!
					{
						(i > 0).then(|| rsx!(hr { class: "row justEven" }))
						div
						{
							class: "row justCenter editSpellWrapper",
							key: "{i}",
							oncontextmenu: move |e| showRemovePopUpWithIndex(e, &clickedX, &clickedY, &showRemove, &lastIndex, i),
							prevent_default: "oncontextmenu",
							
							EditSpell { spell: spell.clone(), spellIndex: i }
						}
					}
				})
				
				div
				{
					class: "new row justCenter",
					input { r#type: "text", value: "", placeholder: "Enter a new Known Spell", onchange: move |e| knownSpellUpdateHandler(&cx, e, None), prevent_default: "oncontextmenu" }
				}
				
				showRemove.then(|| rsx!{
					div
					{
						class: "removePopUpWrapper column justEven",
						style: "left: {posX}px; top: {posY}px;",
						onclick: move |e| hideRemovePopUp(e, &showRemove),
						prevent_default: "oncontextmenu",
						
						div
						{
							class: "removePopUp column justEven",
							
							div { class: "row justEven", "Are you sure you want to remove this Known Spell?" }
							div
							{
								class: "row justEven",
								
								button { onclick: move |e| { hideRemovePopUp(e, &showRemove); knownSpellRemoveHandler(&cx, *(lastIndex.get())); }, prevent_default: "oncontextmenu", "Remove" }
								button { onclick: move |e| hideRemovePopUp(e, &showRemove), prevent_default: "oncontextmenu", "Cancel" }
							}
						}
					}
				})
			}
		}
	});
}

fn knownSpellRemoveHandler(cx: &Scope<KnownSpellsProps>, index: usize)
{
	
}

fn knownSpellUpdateHandler(cx: &Scope<KnownSpellsProps>, event: FormEvent, index: Option<usize>)
{
	
}

// --------------------------------------------------

#[derive(PartialEq, Props)]
pub struct PreparableSpellProps
{
	pub label: String,
	pub prepared: bool,
}

/// The UI Component defining the layout of one of a D&D5e Adventurer's Preparable Spells.
pub fn PreparableSpell(cx: Scope<PreparableSpellProps>) -> Element
{
	let tooltip = match cx.props.prepared
	{
		true => "Prepared".to_string(),
		false => "Not Prepared".to_string(),
	};
	
	return cx.render(rsx!
	{
		div
		{
			class: "row justStart preparableSpell",
			
			CheckCircle { checked: cx.props.prepared, tooltip: tooltip }
			div { class: "spellName", "{cx.props.label}" }
		}
	});
}

// --------------------------------------------------

#[derive(PartialEq, Props)]
pub struct PreparedSpellsProps
{
	pub spells: Vec<Spell>,
}

pub fn PreparedSpells(cx: Scope<PreparedSpellsProps>) -> Element
{
	return cx.render(rsx!
	{
		div
		{
			class: "column justEven preparedSpells",
			
			h3 { class: "row justEven", "Prepared Spells" }
			
			div
			{
				class: "row justEven",
				
				(0..5).map(|i|
				{
					if cx.props.spells.iter().filter(|spell| spell.level == i).next() != None
					{
						let spellLevel = match i > 0
						{
							true => format!("Level {}", i),
							false => "Cantrips".to_string(),
						};
						
						rsx!(cx, div
						{
							class: "column justStart",
							
							div { class: "spellLevel", "{spellLevel}" }
							cx.props.spells.iter()
								.filter(|spell| spell.level == i)
								.map(|spell| rsx!(PreparableSpell { label: spell.name.clone(), prepared: false }))
						})
					}
					else
					{
						None
					}
				})
			}
			
			div
			{
				class: "row justEven",
				
				(5..=9).map(|i|
				{
					if cx.props.spells.iter().filter(|spell| spell.level == i).next() != None
					{
						rsx!(cx, div
						{
							class: "column justStart",
							
							div { class: "spellLevel", "Level {i}" }
							cx.props.spells.iter()
								.filter(|spell| spell.level == i)
								.map(|spell| rsx!(PreparableSpell { label: spell.name.clone(), prepared: false }))
						})
					}
					else
					{
						None
					}
				})
			}
		}
	});
}

// --------------------------------------------------
/// The UI Component defining the layout of a D&D5e Adventurer's character details.
pub fn SpellSlots(cx: Scope) -> Element
{
	let classLevelsRef = use_atom_ref(&cx, AdventurerClassLevels);
	let casterLevel = calculateCasterLevel(classLevelsRef.read().clone());
	let slots = getSpellSlots(casterLevel);
	
	return cx.render(rsx!
	{
		div
		{
			class: "column justStart",
			
			h3 { class: "row justEven", "Spell Slots" }
			
			div
			{
				class: "row justEven",
				
				slots.iter()
					.filter(|(_, slots)| **slots > 0)
					.map(|(level, slots)| rsx!(Track { class: "column".to_string(), label: level.to_string(), tracker: Tracker::new(*slots) }))
			}
		}
	});
}