#!/bin/bash
# R6 U4 native capture helpers. Real Safari/Chrome windows on the real built
# site; no settings changes; dedicated window per browser, closed afterwards.
set -u
OUT=/Users/rockie/Documents/gh-rockie/rustify-ui/docs/plan/DOCS-SSG-P4.ui-captures/r6
mkdir -p "$OUT"

bounds() { # $1=process name -> "x y w h"
  osascript <<EOF 2>/dev/null
tell application "System Events"
  tell process "$1"
    if (count of windows) is 0 then return ""
    set p to position of window 1
    set s to size of window 1
    return (item 1 of p as text) & " " & (item 2 of p as text) & " " & (item 1 of s as text) & " " & (item 2 of s as text)
  end tell
end tell
EOF
}

shot() { # $1=process $2=filename
  osascript -e "tell application \"$1\" to activate" 2>/dev/null
  sleep 1
  local b; b=$(bounds "$1")
  if [ -z "$b" ]; then echo "SHOT-FAIL $2 (no window)"; return 1; fi
  screencapture -x -R$(echo "$b" | tr ' ' ',') "$OUT/$2"
  echo "SHOT $2 bounds=$b"
}

axclick() { # $1=process $2=accessible name to find (button/link), clicks first match
  osascript <<EOF 2>&1
tell application "System Events"
  tell process "$1"
    set target to "$2"
    set found to missing value
    set q to {window 1}
    repeat with n from 1 to 14
      set nxt to {}
      repeat with el in q
        try
          set roleName to role of el
          set nm to name of el
          if nm is target and (roleName is "AXButton" or roleName is "AXLink" or roleName is "AXCheckBox") then
            set found to el
            exit repeat
          end if
        end try
        try
          set kids to UI elements of el
          if kids is not missing value then set nxt to nxt & kids
        end try
      end repeat
      if found is not missing value then exit repeat
      set q to nxt
      if (count of q) is 0 then exit repeat
    end repeat
    if found is missing value then
      return "AXCLICK-MISS " & target
    else
      click found
      return "AXCLICK-OK " & target
    end if
  end tell
end tell
EOF
}

nav_chrome() { # $1=url — navigate active tab of our window
  osascript <<EOF
tell application "Google Chrome"
  set URL of active tab of front window to "$1"
end tell
EOF
}

nav_safari() { # $1=url
  osascript <<EOF
tell application "Safari"
  set URL of current tab of front window to "$1"
end tell
EOF
}

key() { # $1=process $2=keycode or text $3=modifiers(e.g. "command down") $4=mode(keystroke|keycode)
  osascript <<EOF
tell application "System Events"
  tell process "$1"
    if "$4" is "keycode" then
      key code $2 using {$3}
    else
      keystroke $2 using {$3}
    end if
  end tell
end tell
EOF
}

paste_text() { # $1=process $2=text — set clipboard and paste
  printf '%s' "$2" | pbcopy
  key "$1" "v" "command down" keystroke
}
"$@"
