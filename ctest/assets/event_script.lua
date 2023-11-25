
-- Event Script For Game Code

local function OnEvent(event, data)
   if event == "GAIN_FOCUS" then
      print("Gain Focus")
   elseif event == "LOSE_FOCUS" then
      print("Lose Focus")
   elseif event == "MOUSE_BUTTON_PRESSED" then
      print("Mouse Button Pressed")
   elseif event == "MOUSE_BUTTON_RELEASED" then
      print("Mouse Button Released")
   elseif event == "MOUSE_BUTTON_DOUBLECLICKED" then
      print("Mouse Button Double Clicked")
   elseif event == "MOUSE_WHEEL_MOVED" then
      print("Mouse Wheel Moved")
   elseif event == "MOUSE_MOVED" then
      print("Mouse Moved")
   elseif event == "KEY_PRESSED" then
      print("Key Pressed")
   elseif event == "KEY_RELEASED" then
      print("Key Released")
   elseif event == "KEY_DOUBLECLICKED" then
      print("Key Double Clicked")
   elseif event == "KEY_DOWN" then
      print("Key Down")
   elseif event == "KEY_UP" then
      print("Key Up")
   elseif event == "KEY_DOWN_RELEASED" then
      print("Key Down Released")
   end
end

-- Call My Fake Ass Function
onEvent("GAIN_FOCUS", {data = "GAIN_FOCUS"})
