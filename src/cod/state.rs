#![allow(non_snake_case, non_upper_case_globals)]

use dioxus::prelude::*;
use serde::{
	Serialize,
	Deserialize,
};
use std::collections::HashMap;
use crate::{
	core::template::StatefulTemplate,
	cod::{
		advantages::{
			BaseAdvantages,
			BaseAdvantageType,
		},
		merits::Merit,
		tracks::{
			Tracker,
			TrackerState,
		},
		traits::{
			BaseAttribute,
			BaseAttributeType,
			BaseSkill,
			BaseSkillType,
		}
	},
};

pub static CharacterAdvantages: AtomRef<BaseAdvantages> = |_| BaseAdvantages::default();
pub static CharacterAspirations: AtomRef<Vec<String>> = |_| Vec::<String>::new();
pub static CharacterAttributes: AtomRef<HashMap<BaseAttributeType, BaseAttribute>> = |_| BaseAttribute::newAllAttributes();
pub static CharacterBeats: AtomRef<Tracker> = |_| Tracker::new(5);
pub static CharacterExperience: Atom<usize> = |_| 0;
pub static CharacterMerits: AtomRef<Vec<Merit>> = |_| Vec::<Merit>::new();
pub static CharacterSkills: AtomRef<HashMap<BaseSkillType, BaseSkill>> = |_| BaseSkill::newAllSkills();
pub static CharacterSpecialties: AtomRef<Vec<String>> = |_| Vec::<String>::new();


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BaseCharacter
{
	#[serde(default)]
	pub advantages: BaseAdvantages,
	#[serde(default)]
	pub aspirations: Vec<String>,
	#[serde(default)]
	pub attributes: HashMap<BaseAttributeType, BaseAttribute>,
	#[serde(default)]
	pub beats: Tracker,
	#[serde(default)]
	pub experience: usize,
	#[serde(default)]
	pub merits: Vec<Merit>,
	#[serde(default)]
	pub skills: HashMap<BaseSkillType, BaseSkill>,
	#[serde(default)]
	pub specialties: Vec<String>,
}

impl StatefulTemplate for BaseCharacter
{
	fn pull<T>(&mut self, cx: &Scope<T>)
	{
		let advantages = use_atom_ref(cx, CharacterAdvantages);
		let aspirations = use_atom_ref(cx, CharacterAspirations);
		let attributes = use_atom_ref(cx, CharacterAttributes);
		let beats = use_atom_ref(cx, CharacterBeats);
		let experience = use_read(cx, CharacterExperience);
		let merits = use_atom_ref(cx, CharacterMerits);
		let skills = use_atom_ref(cx, CharacterSkills);
		let specialties = use_atom_ref(cx, CharacterSpecialties);
		
		self.advantages = advantages.read().clone();
		self.aspirations = aspirations.read().clone();
		self.attributes = attributes.read().clone();
		self.beats = beats.read().clone();
		self.experience = *experience;
		self.merits = merits.read().clone();
		self.skills = skills.read().clone();
		self.specialties = specialties.read().clone();
	}
	
	fn push<T>(&self, cx: &Scope<T>)
	{
		let advantages = use_atom_ref(cx, CharacterAdvantages);
		let aspirations = use_atom_ref(cx, CharacterAspirations);
		let attributes = use_atom_ref(cx, CharacterAttributes);
		let beats = use_atom_ref(cx, CharacterBeats);
		let experience = use_set(cx, CharacterExperience);
		let merits = use_atom_ref(cx, CharacterMerits);
		let skills = use_atom_ref(cx, CharacterSkills);
		let specialties = use_atom_ref(cx, CharacterSpecialties);
		
		(*advantages.write()) = self.advantages.clone();
		(*aspirations.write()) = self.aspirations.clone();
		(*attributes.write()) = self.attributes.clone();
		(*beats.write()) = self.beats.clone();
		experience(self.experience);
		(*merits.write()) = self.merits.clone();
		(*skills.write()) = self.skills.clone();
		(*specialties.write()) = self.specialties.clone();
	}
}

pub fn updateBaseAdvantage<T>(cx: &Scope<T>, advantage: BaseAdvantageType, value: usize)
{
	let advantagesRef = use_atom_ref(&cx, CharacterAdvantages);
	let attributesRef = use_atom_ref(&cx, CharacterAttributes);
	
	let mut advantages = advantagesRef.write();
	let attributes = attributesRef.read();
	
	match advantage
	{
		BaseAdvantageType::Defense => { advantages.defense = value; }
		BaseAdvantageType::Health => { advantages.health.updateMax(value); }
		BaseAdvantageType::Initiative => { advantages.initiative = value; }
		
		BaseAdvantageType::Size =>
		{
			let finalValue = match value < 1
			{
				true => { 1 }
				false => match value > 10
				{
					true => { 10 }
					false => { value }
				}
			};
			let healthMax = attributes[&BaseAttributeType::Stamina].value + finalValue;
			
			advantages.size = finalValue;
			advantages.health.updateMax(healthMax);
		}
		
		BaseAdvantageType::Speed => { advantages.speed = value; }
		BaseAdvantageType::Willpower => { advantages.willpower.updateMax(value); }
	}
}

pub fn updateBaseAttribute<T>(cx: &Scope<T>, attributeType: &BaseAttributeType, value: usize)
{
	let advantagesRef = use_atom_ref(&cx, CharacterAdvantages);
	let attributesRef = use_atom_ref(&cx, CharacterAttributes);
	let skillsRef = use_atom_ref(&cx, CharacterSkills);
	
	let mut advantages = advantagesRef.write();
	let mut attributes = attributesRef.write();
	let skills = skillsRef.read();
	
	attributes.get_mut(attributeType).unwrap().value = value;
	
	match attributeType
	{
		BaseAttributeType::Composure =>
		{
			advantages.initiative = attributes[&BaseAttributeType::Dexterity].value + attributes[&BaseAttributeType::Composure].value;
			advantages.willpower.updateMax(attributes[&BaseAttributeType::Resolve].value + attributes[&BaseAttributeType::Composure].value);
		}
		
		BaseAttributeType::Dexterity =>
		{
			let defense = match attributes[&BaseAttributeType::Dexterity].value <= attributes[&BaseAttributeType::Wits].value
			{
				true => { attributes[&BaseAttributeType::Dexterity].value }
				false => { attributes[&BaseAttributeType::Wits].value }
			} + skills[&BaseSkillType::Athletics].value;
			
			advantages.defense = defense;
			advantages.initiative = attributes[&BaseAttributeType::Dexterity].value + attributes[&BaseAttributeType::Composure].value;
			advantages.speed = advantages.size + attributes[&BaseAttributeType::Dexterity].value + attributes[&BaseAttributeType::Strength].value;
		}
		
		BaseAttributeType::Resolve => { advantages.willpower.updateMax(attributes[&BaseAttributeType::Composure].value + attributes[&BaseAttributeType::Resolve].value); }
		
		BaseAttributeType::Stamina =>
		{
			let size = advantages.size;
			advantages.health.updateMax(size + attributes[&BaseAttributeType::Stamina].value);
		}
		
		BaseAttributeType::Strength => { advantages.speed = advantages.size + attributes[&BaseAttributeType::Dexterity].value + attributes[&BaseAttributeType::Strength].value; }
		
		BaseAttributeType::Wits =>
		{
			let defense = match attributes[&BaseAttributeType::Dexterity].value <= attributes[&BaseAttributeType::Wits].value
			{
				true => { attributes[&BaseAttributeType::Dexterity].value }
				false => { attributes[&BaseAttributeType::Wits].value }
			} + skills[&BaseSkillType::Athletics].value;
			
			advantages.defense = defense;
		}
		
		_ => {}
	}
}

pub fn updateBaseBeats<T>(cx: &Scope<T>, index: usize, overrideValues: bool)
{
	let beatsRef = use_atom_ref(&cx, CharacterBeats);
	let mut beats = beatsRef.write();
	
	updateTrackerState_SingleState(&mut beats, index, TrackerState::Two, overrideValues);
}

pub fn updateBaseSkill<T>(cx: &Scope<T>, skillType: &BaseSkillType, value: usize)
{
	let advantagesRef = use_atom_ref(&cx, CharacterAdvantages);
	let attributesRef = use_atom_ref(cx, CharacterAttributes);
	let skillsRef = use_atom_ref(&cx, CharacterSkills);
	
	let mut advantages = advantagesRef.write();
	let attributes = attributesRef.read();
	let mut skills = skillsRef.write();
	
	skills.get_mut(skillType).unwrap().value = value;
	
	// Handle 
	match skillType
	{
		BaseSkillType::Athletics =>
		{
			let attributeDefense = match attributes[&BaseAttributeType::Dexterity].value <= attributes[&BaseAttributeType::Wits].value
			{
				true => { attributes[&BaseAttributeType::Dexterity].value }
				false => { attributes[&BaseAttributeType::Wits].value }
			};
			
			advantages.defense = attributeDefense + skills[&BaseSkillType::Athletics].value;
		}
		
		_ => {}
	}
}

pub fn updateBaseHealth<T>(cx: &Scope<T>, damageType: TrackerState, remove: bool, index: Option<usize>)
{
	let advantagesRef = use_atom_ref(&cx, CharacterAdvantages);
	let mut advantages = advantagesRef.write();
	
	if remove
	{
		advantages.health.remove(damageType);
	}
	else
	{
		match index
		{
			Some(i) => { advantages.health.update(damageType, i, false); }
			None => { advantages.health.add(damageType); }
		}
	}
}

pub fn updateBaseWillpower<T>(cx: &Scope<T>, index: usize)
{
	let advantagesRef = use_atom_ref(&cx, CharacterAdvantages);
	let mut advantages = advantagesRef.write();
	
	updateTrackerState_SingleState(&mut advantages.willpower, index, TrackerState::Two, false);
}

/// 
pub fn updateTrackerState_SingleState(tracker: &mut Tracker, index: usize, state: TrackerState, overrideValues: bool)
{
	let len = tracker.values.len();
	
	if overrideValues
	{
		tracker.values.truncate(0);
		(0..index).for_each(|_| tracker.add(state));
	}
	else
	{
		if index >= len
		{
			(len..index+1).for_each(|_| tracker.add(state));
		}
		else if index < len
		{
			(index..len).for_each(|_| tracker.remove(state));
			
			// If we're trying to "disable" more than one box, add one back in.
			// People naturally click where they want the checks to stop
			// not one above where they want them to stop.
			// However, this makes clicking the top most checked box act weird
			// thus... the if.
			if len - index > 1
			{
				tracker.add(state);
			}
		}
	}
}
