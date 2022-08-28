using Godot;
using System;
using OCSM;

public class SaveSheet : FileDialog
{
	public string SheetData { get; set; }
	
	public override void _Ready()
	{
		var path = FileSystemUtilities.DefaultSheetDirectory;
		System.IO.Directory.CreateDirectory(System.IO.Path.GetDirectoryName(path));
		CurrentDir = path;
		Connect(Constants.Signal.FileSelected, this, nameof(doSave));
	}
	
	private void doSave(string filePath)
	{
		var path = filePath;
		if(String.IsNullOrEmpty(CurrentFile) || CurrentFile.Equals(Constants.SheetFileExtension))
		{
			var extensionIndex = path.FindLast(Constants.SheetFileExtension);
			path = path.Insert(extensionIndex, Constants.NewSheetFileName);
		}
		else if(!path.EndsWith(Constants.SheetFileExtension))
			path += Constants.SheetFileExtension;
		
		if(!String.IsNullOrEmpty(SheetData))
			FileSystemUtilities.WriteString(path, SheetData);
	}
}