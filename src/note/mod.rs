/// Models used for handling notes
pub mod model;
/// Controller to handle the notes
pub mod controller;

#[cfg(test)]
mod tests {
    use model::tests;

    use super::*;

    /*
                    #[test]
                    fn check_getting_note(){
                        tests::check_create_note();
                        let connection = db::connect().get().unwrap();
                        let result: Note = Note::get_by_id(1, &connection);
                        println!("{}", result.title);
                    }*/
}
