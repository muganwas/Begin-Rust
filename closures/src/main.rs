#[derive(Debug)]
enum Shirtcolor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<Shirtcolor>,
}
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Inventory {
    fn giveaway(&self, user_pref: Option<Shirtcolor>) -> Shirtcolor {
        user_pref.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> Shirtcolor {
        let mut red = 0;
        let mut blue = 0;
        for shirt in self.shirts.iter() {
            match shirt {
                Shirtcolor::Red => red += 1,
                Shirtcolor::Blue => blue += 1,
            }
        }
        if red > blue {
            Shirtcolor::Red
        } else {
            Shirtcolor::Blue
        }
    }
}
fn main() {
    let shirts = Inventory {
        shirts: vec![Shirtcolor::Red, Shirtcolor::Blue, Shirtcolor::Red],
    };
    let user_pref = Some(Shirtcolor::Blue);
    let mut rectangles = [
        Rectangle {
            width: 3,
            height: 8,
        },
        Rectangle {
            width: 1,
            height: 5,
        },
        Rectangle {
            width: 8,
            height: 17,
        },
    ];
    let mut rv = vec![];
    let rs = String::from("By key value");
    for r in &rectangles {
        // Just to get rid of unused height error
        println!("their heights, {}", r.height);
    }
    rectangles.sort_by_key(|r| {
        rv.push(&rs);
        r.width
    });
    println!("sorted rects {:?}", rectangles);
    let mut a_b = vec![1, 2, 3, 4];
    let only_borrows = || println!("Only borrows {:?}", user_pref);
    let mut add_five = || a_b.push(5);
    //println!("{:?}", a_b); // not allowed because there a current mutable borrow
    add_five();
    only_borrows();
    println!("{:?}", &a_b); // allowed because closure is already done with the vector
    let giveaway_1 = shirts.giveaway(user_pref);
    println!("Giveaway 1: {:?}", giveaway_1);

    let user_pref_1 = None;
    let giveaway_2 = shirts.giveaway(user_pref_1);
    println!("Giveaway 2: {:?}", giveaway_2);
}
