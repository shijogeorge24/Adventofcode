use std::env;
use std::fs;

fn main() {
    // --snip--
//    println!("In file {}", file_path);

    let contents = fs::read_to_string("day1.txt")
        .expect("Should have been able to read the file");

   // println!("With text:\n{contents}");

    let mut sum:i64;
    let mut intiger:i64;
    sum=0;
    intiger=0;
    let mut flag_begin:bool =false;

        let mut temp_int1:i64;
        let mut temp_int2:i64;
        temp_int1=0;
        temp_int2=0;
  for i in 0..contents.len()
  {

 if contents.as_bytes()[i]==10
 {

intiger=(intiger*10)+temp_int1;
intiger=(intiger*10)+temp_int2;

    println!("{}",intiger);
    sum+=intiger;
    intiger=0;
    temp_int1=0;
    temp_int2=0;
    flag_begin=false;
   
    //println!("{}",intiger);
    println!();
 }


     if contents.as_bytes()[i]>=48 && contents.as_bytes()[i]<=57
     {

    
  
    if flag_begin==false
    {
        temp_int1= i64::from((contents.as_bytes()[i])-48);
        temp_int2= i64::from((contents.as_bytes()[i])-48);


        flag_begin=true;
    }
    else
    {
         
    temp_int2= i64::from((contents.as_bytes()[i])-48);
    }


     

  }




}

if temp_int1!=0
{
intiger=(intiger*10)+temp_int1;
intiger=(intiger*10)+temp_int2;
                                                                                                                            println!("{}",intiger);
    sum+=intiger;
}

 println!("sum={}", sum);

}
