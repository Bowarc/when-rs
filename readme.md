# My attempt at [Dreamberd's when feature](<https://github.com/TodePond/DreamBerd?tab=readme-ov-file#when>) in pure safe rust


## Example:
```rust
struct Player {
    deaths: u32,
    health: u32,
}

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
    }; // setup the listener

    // Method 1, store the guard into a variable, but you'll have to drop it before any effect take place
    {
        let mut lock = player.lock();

        lock.health = 0;
    }
    assert!(player.health == 5);
    assert!(player.deaths == 1);

    // Or you can skip the lock allocation, and have it update instantly
    {
        player.lock().health = 0;

        assert!(player.health == 5);
        assert!(player.deaths == 2);
    }
}
```
