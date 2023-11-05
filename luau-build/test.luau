local object = GameObject.new("Test")
local object2 = GameObject.new("Test2")

object.Transform.LocalPosition = Vector2.new(5, 5)

object2.Transform.Parent = object.Transform
object2.Transform.LocalPosition = Vector2.new(5, 5)


print(object.Transform.LocalPosition)
