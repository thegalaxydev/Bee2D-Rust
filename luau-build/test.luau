
Bee2D.setTitle("Test")
Bee2D.loadTexture("test.png")

local testVector = Vector2.new(1, 1)

Bee2D.bindToDraw(function()
    Bee2D.drawTexture("test.png", 0,0, 0, 1, {255,255,255,255})
end)