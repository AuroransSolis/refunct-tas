forward = "forward"
forwards = "forward"
backward = "backward"
backwards = "backward"
left = "left"
right = "right"
jump = "jump"
crouch = "crouch"
menu = "menu"

local deltatime = 0;

function num(var)
  return var and 1 or 0
end

function math.round(num, numDecimalPlaces)
  local mult = 10^(numDecimalPlaces or 0)
  return math.floor(num * mult + 0.5) / mult
end

function parse(res)
  if res.variant == "exit" then
    error("We must exit")
  else
    return res.data
  end
end

function print(...)
  local res = "";
  for k,v in pairs({...}) do
    if k > 1 then
      res = res .. " "
    end
    res = res .. tostring(v)
  end
  __print(res)
end

function step()
  local evt = parse(__step());
  if evt == "drawhud" then
    if drawhud ~= nil then
      drawhud()
    end
  end
end

function waitfornewgame()
  parse(__wait_for_new_game())
end

function presskey(key)
  parse(__press_key(key))
end

function releasekey(key)
  parse(__release_key(key))
end

function movemouse(x, y)
  parse(__move__mouse(x, y))
end

function getdelta()
  return parse(__get_delta())
end

function setdelta(delta)
    deltatime = delta
end

function getlocation()
  local loc = parse(__get_location())
  return loc[1], loc[2], loc[3]
end

function setlocation(x, y, z)
  parse(__set_location(x, y, z))
end

function getrotation()
  local rot = parse(__get_rotation())
  return rot[1], rot[2], rot[3]
end

function setrotation(pitch, yaw)
  local _, _, roll = getrotation()
  parse(__set_rotation(pitch, yaw, roll))
end

function getvelocity()
  local vel = parse(__get_velocity())
  return vel[1], vel[2], vel[3]
end

function setvelocity(x, y, z)
  parse(__set_velocity(x, y, z))
end

function getacceleration()
  local acc = parse(__get_acceleration())
  return acc[1], acc[2], acc[3]
end

function drawline(startx, starty, endx, endy, red, green, blue, alpha, thickness)
  parse(__draw_line(startx, starty, endx, endy, red, green, blue, alpha, thickness))
end

function drawtext(text, red, green, blue, alpha, x, y, scale, scaleposition)
  parse(__draw_text(text, red, green, blue, alpha, x, y, scale, scaleposition))
end

Frame = {
  forward = false,
  backward = false,
  left = false,
  right = false,
  jump = false,
  crouch = false,
  menu = false,
  degx = 0,
  degy = 0,
}

function Frame:new(o)
  o = o or {}
  setmetatable(o, self)
  self.__index = self
  return o
end

local lastframe = Frame:new()

function execframe(frame)
  -- new input
  if frame.forward and not lastframe.forward then
    presskey(forward)
  end
  if frame.backward and not lastframe.backward then
    presskey(backward)
  end
  if frame.left and not lastframe.left then
    presskey(left)
  end
  if frame.right and not lastframe.right then
    presskey(right)
  end
  if frame.jump and not lastframe.jump then
    presskey(jump)
  end
  if frame.crouch and not lastframe.crouch then
    presskey(crouch)
  end
  if frame.menu and not lastframe.menu then
    presskey(menu)
  end

  -- old inputs
  if lastframe.forward and not frame.forward then
    releasekey(forward)
  end
  if lastframe.backward and not frame.backward then
    releasekey(backward)
  end
  if lastframe.left and not frame.left then
    releasekey(left)
  end
  if lastframe.right and not frame.right then
    releasekey(right)
  end
  if lastframe.jump and not frame.jump then
    releasekey(jump)
  end
  if lastframe.crouch and not frame.crouch then
    releasekey(crouch)
  end
  if lastframe.menu and not frame.menu then
    releasekey(menu)
  end

  -- rotation
  if frame.degx ~= 0 or frame.degy ~= 0 then
    local pitch, yaw, _ = getrotation()
    setrotation(pitch + frame.degy, yaw + frame.degx)
  end

  lastframe = frame

  if deltatime ~= 0 then
    setdelta(deltatime)
  end

  step()
end

function frame(keys, degx, degy, repeatnum)
  keys = keys or {}
  degx = degx or 0
  degy = degy or 0
  degy = -degy
  repeatnum = repeatnum or 1
  local pitch, yaw, _ = getrotation()
  startx = yaw
  starty = pitch

  for i=1,repeatnum do
    local currentframe = Frame:new()
    for _, key in pairs(keys) do
      currentframe[key] = true
    end
    if degx ~= 0 or degy ~= 0 then
      framesleft = repeatnum - i + 1
      local pitch, yaw, _ = getrotation()
      remainingx = startx + degx - yaw
      remainingy = starty + degy - pitch
      currentframe.degx = remainingx / framesleft
      currentframe.degy = remainingy / framesleft
    end
    execframe(currentframe)
  end
end


function printstats()
  local x, y, z = getlocation()
  local velx, vely, velz = getvelocity()
  local pitch, yaw, roll = getrotation()
  local accx, accy, accz = getacceleration()
  print(string.format("x: %-6.2f\ty: %-6.2f\tz: %-6.2f\tvelx: %-6.2f\tvely: %-6.2f\tvelz: %-6.2f\tpitch: %-6.2f\tyaw: %-6.2f\troll: %-6.2f\taccx: %-6.2f\taccy: %-6.2f\taccz: %-6.2f\t",
		x, y, z, velx, vely, velz, pitch, yaw, roll, accx, accy, accz))
end
