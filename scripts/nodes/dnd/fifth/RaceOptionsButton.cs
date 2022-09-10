using Godot;
using System;
using OCSM.DnD.Fifth.Meta;
using OCSM.Nodes.Autoload;

namespace OCSM.Nodes.DnD.Fifth
{
	public class RaceOptionsButton : OptionButton
	{
		private MetadataManager metadataManager;
		
		public override void _Ready()
		{
			metadataManager = GetNode<MetadataManager>(Constants.NodePath.MetadataManager);
			metadataManager.Connect(nameof(MetadataManager.MetadataSaved), this, nameof(refreshMetadata));
			metadataManager.Connect(nameof(MetadataManager.MetadataLoaded), this, nameof(refreshMetadata));
			
			refreshMetadata();
		}
		
		private void refreshMetadata()
		{
			if(metadataManager.Container is DnDFifthContainer dfc)
			{
				var index = Selected;
				
				Clear();
				
				AddItem("");
				foreach(var race in dfc.Races)
				{
					AddItem(race.Name);
				}
				
				Selected = index;
			}
		}
	}
}