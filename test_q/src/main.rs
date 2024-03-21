/**fn main() {
    let mut v: Vec::<isize> = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    if let Some(val) = v.get(20){
        println!("{}", val);
    }
}
*/
/**fn main(){
    let v = vec![0,1,2,3,4,5,6,7,8,9];
    let total: i32 = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 4)
    .fold(0, |total, x| total + x);
println!("{:?}", total);
}*/
fn main(){
    let v = vec![0,1,2,3];
    let mut iter = v.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.last());
}