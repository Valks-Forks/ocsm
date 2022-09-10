using Godot;
using System;
using System.Collections.Generic;
using OCSM.DnD.Fifth;

namespace OCSM.Nodes.DnD.Fifth
{
	public class AbilityNode : Container
	{
		[Signal]
		public delegate void AbilityChanged(Transport<Ability> transport);
		
		private sealed class Names
		{
			public const string Name = "Name";
			public const string Score = "Score";
			public const string Modifier = "Modifier";
			public const string SavingThrow = "SavingThrow";
			public const string Skills = "Skills";
		}
		
		public Ability Ability { get; set; }
		public int ProficiencyBonus { get; set; } = 2;
		
		private Label label;
		private SpinBox score;
		private SpinBox modifier;
		private Container skillsContainer;
		private Skill savingThrow;
		
		public override void _Ready()
		{
			label = GetNode<Label>(NodePathBuilder.SceneUnique(Names.Name));
			score = GetNode<SpinBox>(NodePathBuilder.SceneUnique(Names.Score));
			modifier = GetNode<SpinBox>(NodePathBuilder.SceneUnique(Names.Modifier));
			skillsContainer = GetNode<Container>(NodePathBuilder.SceneUnique(Names.Skills));
			savingThrow = GetNode<Skill>(NodePathBuilder.SceneUnique(Names.SavingThrow));
			savingThrow.trackAbility(this);
			savingThrow.Connect(nameof(Skill.ProficiencyChanged), this, nameof(savingThrowChanged));
			
			score.Connect(Constants.Signal.ValueChanged, this, nameof(scoreChanged));
		}
		
		public void refresh()
		{
			if(Ability is Ability)
			{
				label.Text = Ability.Name;
				score.Value = Ability.Score;
				modifier.Value = Ability.Modifier;
				savingThrow.setProficiency(Ability.SavingThrow);
				renderSkills();
			}
		}
		
		private void calculateModifier()
		{
			modifier.Value = Ability.Modifier;
			if(modifier.Value >= 0)
				modifier.Prefix = "+";
			else
				modifier.Prefix = String.Empty;
		}
		
		private void renderSkills()
		{
			foreach(Node child in skillsContainer.GetChildren())
			{
				child.QueueFree();
			}
			
			if(Ability is Ability)
			{
				var resource = ResourceLoader.Load<PackedScene>(Constants.Scene.DnD.Fifth.Skill);
				foreach(var skill in Ability.Skills)
				{
					var instance = resource.Instance<Skill>();
					instance.AbilityModifier = Ability.Modifier;
					instance.ProficiencyBonus = ProficiencyBonus;
					instance.Label = skill.Name;
					instance.Name = skill.Name;
					instance.trackAbility(this);
					instance.Connect(nameof(Skill.ProficiencyChanged), this, nameof(proficiencyChanged), new Godot.Collections.Array(new Transport<OCSM.DnD.Fifth.Skill>(skill)));
					skillsContainer.AddChild(instance);
					instance.setProficiency(skill.Proficient);
				}
			}
		}
		
		private void proficiencyChanged(string currentState, Transport<OCSM.DnD.Fifth.Skill> transport)
		{
			var proficiency = ProficiencyUtility.fromStatefulButtonState(currentState);
			transport.Value.Proficient = proficiency;
			if(Ability.Skills.Find(s => s.Name.Equals(transport.Value.Name)) is OCSM.DnD.Fifth.Skill skill)
				skill.Proficient = proficiency;
			EmitSignal(nameof(AbilityChanged), new Transport<Ability>(Ability));
		}
		
		private void savingThrowChanged(string currentState)
		{
			Ability.SavingThrow = ProficiencyUtility.fromStatefulButtonState(currentState);
			EmitSignal(nameof(AbilityChanged), new Transport<Ability>(Ability));
		}
		
		private void scoreChanged(float value)
		{
			Ability.Score = (int)value;
			calculateModifier();
			EmitSignal(nameof(AbilityChanged), new Transport<Ability>(Ability));
		}
	}
}