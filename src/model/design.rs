use super::super::schema::design;

#[derive(Queryable)]
pub struct Design {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name = "design"]
pub struct NewDesign<'a> {
    pub title: &'a str,
}