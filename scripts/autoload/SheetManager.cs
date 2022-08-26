using Godot;
using System;
using System.Collections.Generic;
using System.Text;
using System.Text.Json;
using OCSM;

public class SheetManager : Node
{
	public void addNewSheet(string scenePath, string name, string json = null)
	{
		if(!String.IsNullOrEmpty(scenePath) && !String.IsNullOrEmpty(name))
		{
			var resource = GD.Load<PackedScene>(scenePath);
			var instance = resource.Instance();
			instance.Name = name;
			
			var target = GetNode(Constants.NodePath.SheetTabs);
			if(target is TabContainer tc)
			{
				var dupeCount = 0;
				foreach(Node c in tc.GetChildren())
				{
					if(c.Name.Contains(instance.Name))
						dupeCount++;
				}
				
				if(dupeCount > 0)
					instance.Name = String.Format("{0} ({1})", instance.Name, dupeCount);
				
				if(!String.IsNullOrEmpty(json))
					((ICharacterSheet)instance).SetJsonData(json);
				
				tc.AddChild(instance);
				tc.CurrentTab = tc.GetTabCount() - 1;
			}
		}
	}
	
	public void closeActiveSheet()
	{
		var tc = GetNode<TabContainer>(PathBuilder.SceneUnique(AppRoot.SheetTabsName, Constants.NodePath.AppRoot));
		if(tc is TabContainer)
		{
			var tab = tc.GetCurrentTabControl();
			if(tab is Node)
			{
				if(tc.GetTabCount() <= tc.CurrentTab + 1)
					showNewSheetUI();
				tab.QueueFree();
			}
		}
	}
	
	public string getActiveSheetJsonData()
	{
		string data = null;
		var tc = GetNode<TabContainer>(PathBuilder.SceneUnique(AppRoot.SheetTabsName, Constants.NodePath.AppRoot));
		if(tc is TabContainer)
		{
			var tab = tc.GetCurrentTabControl();
			if(tab is ICharacterSheet sheet)
			{
				data = sheet.GetJsonData();
			}
		}
		return data;
	}
	
	public void hideNewSheetUI()
	{
		if(GetNode<Control>(Constants.NodePath.SheetTabs) is Control sheetTabs && !sheetTabs.Visible)
			sheetTabs.Show();
		if(GetNodeOrNull<Control>(Constants.NodePath.NewSheet) is Control newSheet)
			newSheet.QueueFree();
	}
	
	public void loadSheetJsonData(string json)
	{
		if(!String.IsNullOrEmpty(json))
		{
			var tc = GetNode<TabContainer>(PathBuilder.SceneUnique(AppRoot.SheetTabsName, Constants.NodePath.AppRoot));
			if(tc is TabContainer)
			{
				var loaded = false;
				if(json.Contains(OCSM.GameSystem.Cod.Changeling))
				{
					addNewSheet(Constants.Scene.CoD.Changeling.Sheet, Constants.Scene.CoD.Changeling.NewSheetName, json);
					loaded = true;
				}
				else if(json.Contains(OCSM.GameSystem.Cod.Mortal))
				{
					addNewSheet(Constants.Scene.CoD.Mortal.Sheet, Constants.Scene.CoD.Mortal.NewSheetName, json);
					loaded = true;
				}
				
				if(loaded)
					hideNewSheetUI();
			}
		}
	}
	
	public void showNewSheetUI()
	{
		var existingNode = GetNodeOrNull<NewSheet>(Constants.NodePath.NewSheet);
		if(!(existingNode is NewSheet))
		{
			var sheetTabsNode = GetNode<TabContainer>(Constants.NodePath.SheetTabs);
			sheetTabsNode.Hide();
			
			var resource = GD.Load<PackedScene>(Constants.Scene.NewSheet);
			var instance = resource.Instance<NewSheet>();
			GetNode<Control>(Constants.NodePath.AppRoot).AddChild(instance);
		}
	}
}
