using System;
using System.Collections.Generic;

namespace OCSM.DnD.Fifth
{
	public class FifthAdventurer : Character
	{
		public List<AbilityScore> AbilityScores { get; set; }
		public string Alignment { get; set; }
		public Background Background { get; set; }
		public bool BardicInspiration { get; set; }
		public string Bonds { get; set; }
		public List<Class> Classes { get; set; }
		public List<Feature> Features { get; set; }
		public string Flaws { get; set; }
		public string Ideals { get; set; }
		public bool Inspiration { get; set; }
		public string PersonalityTraits { get; set; }
		public Race Race { get; set; }
		public List<SavingThrow> SavingThrows { get; set; }
		public List<Skill> Skills { get; set; }
		
		public FifthAdventurer() : base(OCSM.GameSystem.DnD.Fifth)
		{
			AbilityScores = AbilityScore.generateBaseAbilityScores();
			Alignment = String.Empty;
			Background = null;
			BardicInspiration = false;
			Bonds = String.Empty;
			Classes = new List<Class>();
			Features = new List<Feature>();
			Flaws = String.Empty;
			Ideals = String.Empty;
			Inspiration = false;
			PersonalityTraits = String.Empty;
			Race = null;
			SavingThrows = SavingThrow.generateBaseSavingThrows();
			Skills = Skill.generateBaseSkills();
		}
	}
}
