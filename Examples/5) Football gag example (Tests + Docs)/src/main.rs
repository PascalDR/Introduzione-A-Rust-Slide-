/// ball_maker_a is a function that concat to a ball type the string "Ball" 
fn ball_maker_a( ball_type : &mut String ) -> &String{
    ball_type.push_str("Ball");
    ball_type
}

/// ball_maker_b is a function that concat to a ball type string the a ball name string
pub fn ball_maker_b<'a>( ball_type : &'a mut String, ball_name : &String ) -> &'a String{
    ball_type.push_str(ball_name);
    ball_type
}

/// football_checker is the function that checks two reference and return one of this two 
#[doc(hidden)]
pub fn football_checker<'a>( sa : &'a String, sb : &'a String ) -> &'a String {
    if sa == &"FootBall".to_string(){
        sa
    }else{
        sb
    }
}

pub fn lucy_hold_the_football( football: &String ){
    //! lucy_hold_the_football is the function that permits Lucy to hold the Football
    println!("Lucy hold the {}", football);
}

/// # charlie_kick_the_football
/// ## The most important function in the code
/// You can call it as
/// ```
/// charlie_kick_the_football( &"FootBall".to_string() );
/// ```
pub fn charlie_kick_the_football( football: &String ){
    println!("Carlie Brown kick the {}", football);
}

fn main() {
    let mut ball_type_basket = "Basket".to_string();
    let ball_a : &String = ball_maker_a( &mut ball_type_basket );

    let mut ball_type_foot   = "Foot".to_string();
    let ball_b : &String;
    {
        let ball_name = "Ball".to_string();
        ball_b = ball_maker_b( &mut ball_type_foot, &ball_name );
    }

    let football = football_checker( ball_a, ball_b );

    lucy_hold_the_football(football);
    charlie_kick_the_football(football);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_football() {
        let mut ball_type = "Foot".to_string();
        let ball : &String;
        {
            let ball_name = "Ball".to_string();
            ball = ball_maker_b( &mut ball_type, &ball_name );
        }
        assert_eq!(ball, &"FootBall".to_string());
    }

    #[test]
    fn return_basketball() {
        let mut ball_type_basket = "Basket".to_string();
        let ball_a : &String = ball_maker_a( &mut ball_type_basket );

        let mut ball_type_foot   = "Foot".to_string();
        let ball_b : &String;
        {
            let ball_name = "Ball".to_string();
            ball_b = ball_maker_b( &mut ball_type_foot, &ball_name );
        }

        let football = football_checker( ball_a, ball_b );
        assert_eq!(football, &"BasketBall".to_string());
    }
}