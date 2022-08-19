using Godot;
using System;
using System.Collections.Generic;
using OCSM;

public class ItemList : ScrollContainer
{
	private const string InputContainer = "Column";
	
	[Signal]
	public delegate void ValueChanged(List<string> values);
	
	public List<string> Values { get; set; } = new List<string>();
	
	public override void _Ready()
	{
		refresh();
	}
	
	public void refresh()
	{
		foreach(Node c in GetNode<VBoxContainer>(InputContainer).GetChildren())
		{
			c.QueueFree();
		}
		
		foreach(var v in Values)
		{
			if(!String.IsNullOrEmpty(v))
				addInput(v);
		}
		
		addInput();
	}
	
	private void textChanged(string text)
	{
		var values = new List<string>();
		var children = GetNode<VBoxContainer>(InputContainer).GetChildren();
		foreach(LineEdit c in children)
		{
			if(!String.IsNullOrEmpty(c.Text))
				values.Add(c.Text);
			else if(children.IndexOf(c) != children.Count - 1)
				c.QueueFree();
		}
		
		EmitSignal(nameof(ValueChanged), values);
		
		if(children.Count <= values.Count)
		{
			addInput();
		}
	}
	
	private void addInput(string value = "")
	{
		var container = GetNode<VBoxContainer>(InputContainer);
		var node = new LineEdit();
		node.Text = value;
		node.SizeFlagsHorizontal = (int)Control.SizeFlags.ExpandFill;
		node.SizeFlagsVertical = (int)Control.SizeFlags.ExpandFill;
		node.RectMinSize = new Vector2(0, 25);
		node.HintTooltip = "Enter a new " + Name.Substring(0, Name.Length - 1);
		container.AddChild(node);
		node.Connect(Constants.Signal.TextChanged, this, nameof(textChanged));
	}
}
