struct Point{
    x: i32,
    y: i32,
}

struct SquarePos{
    left_up_pos : Point,
    right_down_pos : Point,
}

fn calc_width_height(sp: SquarePos) -> (i32, i32){
    let width = sp.right_down_pos.x - sp.left_up_pos.x;
    let height = sp.right_down_pos.y - sp.left_up_pos.y;

    (width, height)
}

fn main(){
    let s1 = SquarePos{
        left_up_pos: Point{ x: 0, y: 0}, 
        right_down_pos: Point{x: 3, y: 5}
    };

    let square_size = calc_width_height(s1);
    println!("tuple = {:?}", square_size);
    println!("w = {}, h = {}", square_size.0, square_size.1);
    println!("size = {}", square_size.0 * square_size.1);
}