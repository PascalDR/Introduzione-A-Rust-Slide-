fn ball_maker_a( ball_type : &mut String ) -> &String{
    ball_type.push_str("ball");
    ball_type
}

fn ball_maker_b( ball_type : &mut String, ball_name : &String ) -> &String{
    ball_type.push_str(ball_name);
    ball_type
}

fn football_checker( sa : &String, sb : &String ) -> &String {
    if sa == &"FootBall".to_string(){
        sa
    }else{
        sb
    }
}

/*
fn ball_maker_b<'a>( ball_type : &'a mut String, ball_name : &String ) -> &'a String{
    ball_type.push_str(ball_name);
    ball_type
}

fn football_checker<'a>( sa : &'a String, sb : &'a String ) -> &'a String {
    if sa == &"FootBall".to_string(){
        sa
    }else{
        sb
    }
}*/

fn lucy_hold_the_football( football: &String ){
    println!("Lucy hold the {}", football);
}

fn charlie_kick_the_football( football: &String ){
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