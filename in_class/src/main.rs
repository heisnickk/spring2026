fn get_rgb(c:char) -> (u8, u8, u8) {
  //  if c == 'R' {
  //      (255, 0, 0)
  //  }
  //  else if c == 'G' {
  //      (0, 255, 0)
  //  }
  //  else if c == 'B' {
  //      (0, 0, 255)
 //   }
 //       (0, 0, 0)
    
    match c {
    'R' => (255, 0, 0),
    'G' => (0, 255, 0),
    'B' => (0, 0, 255),
    _   => (0, 0, 0),
}
}


fn main() {
    // we are going to accept a letter like RGB
    // and we should return tuple 
    // RED tuple(255, 0, 0)
    // GREEN tuple (0, 255, 0)
    // BLUE tuple (0, 0, 255)

    //write a function which accepts char 'R', 'G', 'B'
    // and returns above specified tuple

   let res = get_rgb('R');
    println!("{:?}", res);

    let res = get_rgb('G');
    println!("{:?}", res);

    let res = get_rgb('B');
    println!("{:?}", res);
    

    let letters = ['R', 'G', 'B'];

    for l in letters.iter() {
        let res = get_rgb(*l);
        println!("{:?}", res);
    }
    
    for idx in 0..letters.len() {
        let res = get_rgb(letters[idx]);
        println!("{:?}", res);
    }


}

