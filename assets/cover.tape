Output assets/cover.png

Set Shell "bash"
Set FontSize 16
Set Width 1400
Set Height 800
Set Theme "Catppuccin Mocha"
Set Padding 40

Hide
Type "cd /Users/aksharadt/Documents/personal/projects/manrust"
Enter
Type "./target/release/man-tui"
Enter
Show
Sleep 4s
Type "l"
Sleep 300ms
Type "s"
Sleep 2s
Enter
Sleep 500ms
Enter
Sleep 3s
Screenshot assets/cover.png
Type "q"
