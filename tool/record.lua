require "prelude"

print("record")
__start_recording_replay("uiae")
print("started")
for i=1,500,1 do
  step()
end
__stop_recording_replay()
print("stopped")
__play_replay("uiae")
