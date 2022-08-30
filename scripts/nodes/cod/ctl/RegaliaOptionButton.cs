using Godot;
using OCSM;
using OCSM.CoD.CtL.Meta;

public class RegaliaOptionButton : OptionButton
{
	[Export]
	public bool emptyOption = true;
	[Export]
	private bool includeNonRegalia = false;
	
	public override void _Ready()
	{
		if(emptyOption)
			AddItem("");
		
		var container = GetNode<MetadataManager>(Constants.NodePath.MetadataManager).Container;
		if(container is CoDChangelingContainer ccc)
		{
			foreach(var regalia in ccc.Regalias)
			{
				AddItem(regalia.Name);
			}
		}
	}
}
