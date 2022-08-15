using Godot;
using System;

public class CircleState : TextureButton
{
	
	public bool CurrentState { get; set; } = false;
	[Export]
	private bool HandleMouseEvents { get; set; } = true;
	
	public override void _Ready()
	{
		updateTexture();
		
		if(HandleMouseEvents)
			Connect(Constants.Signal.GuiInput, this, nameof(handleClick));
	}
	
	public void toggleState()
	{
		CurrentState = !CurrentState;
	}
	
	public void updateTexture()
	{
		var tex = Constants.Texture.FullTransparent;
		if(CurrentState)
			tex = Constants.Texture.TrackCircle1;
		TextureNormal = GD.Load<StreamTexture>(tex);
	}
	
	private void handleClick(InputEvent e)
	{
		if(e is InputEventMouseButton buttonEvent && buttonEvent.Pressed)
		{
			toggleState();
			updateTexture();
		}
	}
}
