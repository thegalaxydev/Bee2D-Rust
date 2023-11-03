-- Bee2D by thegalaxydev

--- @class Vector2

local Vector2 = {}

export type Vector2 = {
    X: number,
    Y: number
} & typeof(Vector2)

function Vector2.new(X: number, Y: number)
    local self = setmetatable({
        X = X or 0,
        Y = Y or 0,

        Magnitude = math.sqrt(X * X + Y * Y),
        Unit = Vector2.new(X / math.sqrt(X * X + Y * Y), Y / math.sqrt(X * X + Y * Y))
    }, 
    {
        __index = Vector2,
        __class = "Vector2",

        __tostring = function(self)
            return string.format("{ %s, %s }", self.X, self.Y)
        end,

        __add = function(lhs, rhs) : Vector2
            assert(type(rhs) == 'table', "Invalid type for Vector2 addition!")

            return Vector2.new(lhs.X + rhs.X, lhs.Y + rhs.Y)
            
        end,

        __sub = function(lhs, rhs) : Vector2
            assert(type(rhs) == 'table', "Invalid type for Vector2 subtraction!")


            return Vector2.new(lhs.X - rhs.X, lhs.Y - rhs.Y)
        end,

        __mul = function(lhs, rhs) : Vector2
            assert(type(rhs) == 'number' or type(rhs) == 'table', "Invalid type for Vector2 multiplication!")

            if type(rhs) == 'number' then
                return Vector2.new(lhs.X * rhs, lhs.Y * rhs)
            else
                return Vector2.new(lhs.X * rhs.X, lhs.Y * rhs.Y)
            end
        end,

        __div = function(lhs, rhs) : Vector2
            assert(type(rhs) == 'number' or type(rhs) == 'table', "Invalid type for Vector2 division!")

            if type(rhs) == 'number' then
                assert(rhs ~= 0, "Division by zero!")
                return Vector2.new(lhs.X / rhs, lhs.Y / rhs)
            else
                assert(rhs.X ~= 0 and rhs.Y ~= 0, "Division by zero vector!")
                return Vector2.new(lhs.X / rhs.X, lhs.Y / rhs.Y)
            end
        end,
    })

    return table.freeze(self)
end


Vector2.zero = Vector2.new(0, 0)
Vector2.one = Vector2.new(1, 1)
Vector2.XAxis = Vector2.new(1, 0)
Vector2.YAxis = Vector2.new(0, 1)

function Vector2:Lerp(b: Vector2, t: number) : Vector2
    return Vector2.new(self.X + (b.X - self.X) * t, self.Y + (b.Y - self.Y) * t)
end

function Vector2:Dot(b: Vector2) : number
    return self.X * b.X + self.Y * b.Y
end

function Vector2:Cross(b: Vector2) : number
    return self.X * b.Y - self.Y * b.X
end

function Vector2.Max(...: Vector2) : Vector2
    local max = Vector2.new(-math.huge, -math.huge)

    for _, v in pairs({...}) do
        if v.X > max.X then
            max.X = v.X
        end

        if v.Y > max.Y then
            max.Y = v.Y
        end
    end

    return max
end

function Vector2.Min(...: Vector2) : Vector2
    local min = Vector2.new(math.huge, math.huge)

    for _, v in pairs({...}) do
        if v.X < min.X then
            min.X = v.X
        end

        if v.Y < min.Y then
            min.Y = v.Y
        end
    end

    return min
end

return Vector2