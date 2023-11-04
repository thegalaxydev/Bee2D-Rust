
Bee2D.setTitle("Test")
Bee2D.loadTexture("test.png")

local testVector = Vector2.new(1, 1)

Bee2D.bindToDraw(function()
    Bee2D.drawTexture("test.png", 0,0, 0, 1, {255,255,255,255})

    Bee2D.drawRectangle(0,0, 100, 100, {255,0,0,255})
end)


local test = Vector2.new(5,20)
print(test.Magnitude)


print(Matrix3.new(1,2,3,4,5,6,7,8,9) * Matrix3.new(1,2,3,4,5,6,7,8,9))
