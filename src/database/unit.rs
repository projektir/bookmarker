#[derive(Queryable, Debug)]
pub struct Unit {
    id: i32,
    pub name: String,
    pub building: String,
    pub minerals: i32,
    pub gas: i32,
    pub supply: i32,
}
