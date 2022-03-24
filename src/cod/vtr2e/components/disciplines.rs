#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::{
	prelude::*,
	events::FormEvent
};
use crate::cod::{
	components::dots::{
		Dots,
		DotsProps,
	},
	vtr2e::{
		disciplines::{
			DisciplineType,
			Devotion,
			DevotionField,
		},
		state::{
			KindredDisciplines,
			KindredDevotions,
			updateDiscipline,
		}
	}
};

pub fn Disciplines(cx: Scope) -> Element
{
	let disciplinesRef = use_atom_ref(&cx, KindredDisciplines);
	let disciplines = disciplinesRef.read();
	
	let dotsClass = "entry row";
	
	return cx.render(rsx!
	{
		div
		{
			class: "disciplines entryListWrapper column",
			
			div { class: "entryListLabel", "Disciplines" }
			
			div
			{
				class: "entryList column",
				
				Dots { class: dotsClass.to_string(), label: "Animalism".to_string(), max: 5, value: disciplines.animalism.value, handler: dotsHandler, handlerKey: DisciplineType::Animalism }
				Dots { class: dotsClass.to_string(), label: "Auspex".to_string(), max: 5, value: disciplines.auspex.value, handler: dotsHandler, handlerKey: DisciplineType::Auspex }
				Dots { class: dotsClass.to_string(), label: "Celerity".to_string(), max: 5, value: disciplines.celerity.value, handler: dotsHandler, handlerKey: DisciplineType::Celerity }
				Dots { class: dotsClass.to_string(), label: "Dominate".to_string(), max: 5, value: disciplines.dominate.value, handler: dotsHandler, handlerKey: DisciplineType::Dominate }
				Dots { class: dotsClass.to_string(), label: "Majesty".to_string(), max: 5, value: disciplines.majesty.value, handler: dotsHandler, handlerKey: DisciplineType::Majesty }
				Dots { class: dotsClass.to_string(), label: "Nightmare".to_string(), max: 5, value: disciplines.nightmare.value, handler: dotsHandler, handlerKey: DisciplineType::Nightmare }
				Dots { class: dotsClass.to_string(), label: "Obfuscate".to_string(), max: 5, value: disciplines.obfuscate.value, handler: dotsHandler, handlerKey: DisciplineType::Obfuscate }
				Dots { class: dotsClass.to_string(), label: "Protean".to_string(), max: 5, value: disciplines.protean.value, handler: dotsHandler, handlerKey: DisciplineType::Protean }
				Dots { class: dotsClass.to_string(), label: "Resilience".to_string(), max: 5, value: disciplines.resilience.value, handler: dotsHandler, handlerKey: DisciplineType::Resilience }
				Dots { class: dotsClass.to_string(), label: "Vigor".to_string(), max: 5, value: disciplines.vigor.value, handler: dotsHandler, handlerKey: DisciplineType::Vigor }
			}
		}
	});
}

fn dotsHandler(cx: &Scope<DotsProps<DisciplineType>>, clickedValue: usize)
{
	let finalValue = match clickedValue == cx.props.value
	{
		true => { clickedValue - 1 }
		false => { clickedValue }
	};
	
	match &cx.props.handlerKey
	{
		Some(dt) =>
		{
			updateDiscipline(cx, dt, finalValue);
		}
		None => {}
	}
}

// -----

pub fn Devotions(cx: Scope) -> Element
{
	let devotionsRef = use_atom_ref(&cx, KindredDevotions);
	let devotions = devotionsRef.read();
	
	return cx.render(rsx!
	{
		div
		{
			class: "devotions entryListWrapper column",
			
			div { class: "entryListLabel", "Devotions" }
			
			div
			{
				class: "entryList column",
				
				devotions.iter().enumerate().map(|(i, dev)| rsx!(cx, div
				{
					class: "entry column",
					
					div
					{
						class: "row",
						
						div { class: "label first", "Name:" }
						input { r#type: "text", value: "{dev.name}", onchange: move |e| inputHandler(e, &cx, Some(i), DevotionField::Name) }
						div { class: "label second", "Cost:" }
						input { r#type: "text", value: "{dev.cost}", onchange: move |e| inputHandler(e, &cx, Some(i), DevotionField::Cost) }
					}
					
					div
					{
						class: "row",
						
						div { class: "label first", "Dice Pool:" }
						input { r#type: "text", value: "{dev.dicePool}", onchange: move |e| inputHandler(e, &cx, Some(i), DevotionField::DicePool) }
						div { class: "label second", "Action:" }
						input { r#type: "text", value: "{dev.action}", onchange: move |e| inputHandler(e, &cx, Some(i), DevotionField::Action) }
					}
					
					div
					{
						class: "row",
						
						div { class: "label first", "Requirements:" }
						input { r#type: "text", value: "{dev.disciplines}", onchange: move |e| inputHandler(e, &cx, Some(i), DevotionField::Disciplines) }
						div { class: "label second", "Reference:" }
						input { r#type: "text", value: "{dev.reference}", onchange: move |e| inputHandler(e, &cx, Some(i), DevotionField::Reference) }
					}
				}))
				
				div
				{
					class: "entry column",
					
					div
					{
						class: "row",
						
						div { class: "label first", "Name:" }
						input { r#type: "text", value: "", onchange: move |e| inputHandler(e, &cx, None, DevotionField::Name) }
						div { class: "label second", "Cost:" }
						input { r#type: "text", value: "", onchange: move |e| inputHandler(e, &cx, None, DevotionField::Cost) }
					}
					
					div
					{
						class: "row",
						
						div { class: "label first", "Dice Pool:" }
						input { r#type: "text", value: "", onchange: move |e| inputHandler(e, &cx, None, DevotionField::DicePool) }
						div { class: "label second", "Action:" }
						input { r#type: "text", value: "", onchange: move |e| inputHandler(e, &cx, None, DevotionField::Action) }
					}
					
					div
					{
						class: "row",
						
						div { class: "label first", "Requirements:" }
						input { r#type: "text", value: "", onchange: move |e| inputHandler(e, &cx, None, DevotionField::Disciplines) }
						div { class: "label second", "Reference:" }
						input { r#type: "text", value: "", onchange: move |e| inputHandler(e, &cx, None, DevotionField::Reference) }
					}
				}
			}
		}
	});
}

fn inputHandler(e: FormEvent, cx: &Scope, index: Option<usize>, prop: DevotionField)
{
	let devotionsRef = use_atom_ref(&cx, KindredDevotions);
	let mut devotions = devotionsRef.write();
	
	match index
	{
		Some(i) =>
		{
			match prop
			{
				DevotionField::Action => { devotions[i].action = e.value.clone(); }
				DevotionField::Cost => { devotions[i].cost = e.value.clone(); }
				DevotionField::DicePool => { devotions[i].dicePool = e.value.clone(); }
				DevotionField::Disciplines => { devotions[i].disciplines = e.value.clone(); }
				DevotionField::Name => { devotions[i].name = e.value.clone(); }
				DevotionField::Reference => { devotions[i].reference = e.value.clone(); }
			}
		}
		None =>
		{
			match prop
			{
				DevotionField::Action => { devotions.push(Devotion { action: e.value.clone(), ..Default::default() }); }
				DevotionField::Cost => { devotions.push(Devotion { cost: e.value.clone(), ..Default::default() }); }
				DevotionField::DicePool => { devotions.push(Devotion { dicePool: e.value.clone(), ..Default::default() }); }
				DevotionField::Disciplines => { devotions.push(Devotion { disciplines: e.value.clone(), ..Default::default() }); }
				DevotionField::Name => { devotions.push(Devotion { name: e.value.clone(), ..Default::default() }); }
				DevotionField::Reference => { devotions.push(Devotion { reference: e.value.clone(), ..Default::default() }); }
			}
		}
	}
}
