use raster::{Image, Color};

pub struct Point {
    X : i32,
    Y : i32,
}
impl Point  {
	pub fn new(x:i32,y:i32) -> Self {

        Self{
            X:x,
            Y:y,

        }

	}
}
pub struct Line {
    p1 : Point,
    p2 : Point,

}
impl Line {
  pub fn new (point1 : Point , point2 : Point) ->Self{

    Self {
        p1 :point1,
        p2 : point2,
    }


  }

}
pub struct Triangle {
    p1 : Point,
    p2 : Point,
    p3 : Point,
}
impl Triangle {
    pub fn new (point1 : Point , point2 : Point,Point3 : Point) ->Self{
        Self {
            p1 : point1,
            p2 : point2,
            p3 : Point3,
        }
    }
}
pub struct Rectangle {
    pub struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    } 
}
impl Rectangle{
    pub fn new (point1 : Point , point2 : Point) ->Self{
        Self {
            top_left : point1,
            bottom_right : point2,
        }
    }
}
pub struct Circle {
    center : Point,
    radius : i32,
}
impl Circle {
    pub fn new(p1 : Point , r : i32)-> {
        Self {
            center : p1,
            radius : r ,
        }
    }
}
