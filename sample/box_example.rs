enum List
{
    Cons(i32, Box<List>),
    End,
}

use List::Cons;
use List::End;

