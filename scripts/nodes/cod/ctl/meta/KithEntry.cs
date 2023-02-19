using Godot;
using OCSM.CoD.CtL;
using OCSM.CoD.CtL.Meta;
using OCSM.Nodes.Meta;

namespace OCSM.Nodes.CoD.CtL.Meta
{
	public partial class KithEntry : BasicMetadataEntry
	{
		protected override void entrySelected(int index)
		{
			var optionsButton = GetNode<OptionButton>(NodePathBuilder.SceneUnique(ExistingEntryName));
			var name = optionsButton.GetItemText(index);
			if(metadataManager.Container is CoDChangelingContainer ccc)
			{
				if(ccc.Kiths.Find(c => c.Name.Equals(name)) is Kith kith)
				{
					loadEntry(kith);
					optionsButton.Selected = 0;
				}
			}
		}
		
		public override void refreshMetadata()
		{
			if(metadataManager.Container is CoDChangelingContainer ccc)
			{
				var optionButton = GetNode<OptionButton>(NodePathBuilder.SceneUnique(ExistingEntryName));
				optionButton.Clear();
				optionButton.AddItem("");
				foreach(var c in ccc.Kiths)
				{
					optionButton.AddItem(c.Name);
				}
			}
		}
	}
}
