on nativePlayer(nativeName)
	if not (running of application nativeName) then return "not running"
	if nativeName is "Spotify" then
		tell application "Spotify"
			if player state is stopped then return "stopped"
			set trackArtist to artist of current track
			set trackName to name of current track
			if player state is paused then
				return trackArtist & " - " & trackName
			else
				return trackArtist & " - " & trackName
			end if
			
		end tell
	else if nativeName is "Music" then
		tell application "Music"
			if player state is stopped then return "stopped"
			set trackArtist to artist of current track
			set trackName to name of current track
			if player state is paused then
				return trackArtist & " - " & trackName
			else
				return trackArtist & " - " & trackName
			end if
			
		end tell
	end if
end nativePlayer


nativePlayer("Spotify")
