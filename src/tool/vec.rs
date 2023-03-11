use druid::Vec2;

pub fn mul_vec(one: &Vec2, two: &Vec2) -> Vec2 {
    return Vec2::new(
        one.x * two.x,
        one.y * two.y
    );
}