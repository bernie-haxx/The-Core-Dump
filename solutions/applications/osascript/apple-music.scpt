on appleTrack()
	tell application "Tidal" 
		set artistName to artist of current track 
		set trackName to name of current track
		set t to artistName & " - " & trackName
		return t
	end tell
end appleTrack


on appleRewind()
	tell application "Tidal" to set player position to 0
end appleRewind

on applePlay()
	tell application "Tidal" to play
end applePlay

on applePause()
	tell application "Tidal" to stop current track 
end applePause

appleTrack()
