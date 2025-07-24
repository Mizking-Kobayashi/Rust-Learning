const SIZE_OF_SQUARE : u32 = 64;

fn main(){
    //変更する変数にはmutをつける
    let mut count = 0u64;

    count += 1;

    println!("count = {}", count);

    let mut shadowing = 32u8;
    {
        let shadowing = shadowing;

        //error!!
        // shadowing = 1u8;

        println!("shadowing = {}", shadowing);
    }

    shadowing = 16u8;

    println!("shadowing = {}", shadowing);
    println!("Size of square was defined as {}", SIZE_OF_SQUARE);
}