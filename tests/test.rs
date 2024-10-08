struct Player {
    deaths: u32,
    health: u32,
}

#[test]
fn main() {
    use ::when::when;

    let player = Player {
        deaths: 0,
        health: 5,
    };

    when! {
        (player.health == 0) {
            println!("You lose");
            player.deaths +=1;
            player.health = 5;
        },
        player
    };

    {
        let mut lock = player.lock();

        lock.health = 0;
    }

    assert!(player.health == 5);
    assert!(player.deaths == 1);
    {
        player.lock().health = 0;

        assert!(player.health == 5);
        assert!(player.deaths == 2);
    }
}
