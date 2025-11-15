fn main(){
    let text1 = "Hola";
    let text2 = "Mundo";

    let result = longer(text1,text2);
    println!("Larger text is {}", result);   
}

fn longer<'a>(val1: &'a str, val2: &'a str) -> &'a str
{
    if val1.len() > val2.len()
    {
        val1
    }else
    {
        val2
    }
}