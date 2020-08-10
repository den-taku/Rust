/// A trait for characters, items, and scenery -
/// anything in the game world that's visible on screen.
trait Visible {
   /// Render this object on the given canvas.
   fn draw(&self, canvas: &mut Canvas);

   /// Return true if clicking at (x,y) should
   /// select this object.
   fn hit_test(&self, x: i32, y: i32) ->bool;
}

/// Someone in the game world, either the player or some other
/// pixie, gargoyle, squirrel, ogre, etc.
trait Creature: Visible {
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> Direction;
}

impl Broom {
    /// Helper function used by Broom::draw() below.
    fn broomstick_range(&self) -> Range<i32> {
        self.y - self.hight - 1 .. self.y
    }
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.broomstick_range() {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    } 

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x
        && self.y - self.height - 1 <= y
        && y <= self.y
    }
}

fn main() {
    println!("Hello, world!");
}
