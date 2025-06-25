u need to compile everytime after every change 
primitive datatypes or scalar datatypes
rust is statically typed language 
int float bool char

Integer rust has signed ( + and -) and unsigned integer only has +types of diff sizes

i8,i16,i32,i64,i128 : signed integers could be positive or negative
u8, u16, u32, u64,u128 : unsigned integers are only positive

diff between i8,i16...i128 smaller the number smaller the range they can have

even if we use let keyword variables are by default immutable in rust to make it mutable use mut keyword

shadowing - we can redefine the same variable with same name in same scope in rust so its intersting

if i define a variable as a mut one then we can change the value but not the type that means x here should be indeed an integer only so yeah trickyy

constants  We need to definetly define the typeof variable while using const also declaring and initialised at the same time in rust also shadowing doesnt work in const
also mutable doesnt work 

ownership in rust

before knowing what is ownership lets see something called memory handling

all langs handle and manage memory in there own way while running a program 

Approach 1 - garbage collector that constantly looks for no longer used memory as program runs
Approach 2 - langs like c cpp assembly use explicitly allocate/free memory
Approach 3 - In rust memory is managed through a set of rules that compiler checks each time program is executed and if any rule is vioalted the program wont compile simple 
ownership doesnt slow down ur runningprogram

the stack and the heap

in rust whether a value is on the stack or the heap affects how the lang behaves and why u must make certain decisions

both the stack and heap are parts of memory available to your code to use at runtime but they are structured in diff ways

as we know stack is stored like a tower last in first out rules is used we can push and pop 

all data stoed on the stack must have known size fixed size 

data which constantly changes its size or one with unknown isze that might cant be stored in stack we can use

heap is less organised than stack when u put data on heap u request some space 

the memory allocator finds a big enough empty spot in the heap . makes it as being in use and returns a pointer in the locations address this process is called allocating

you can store the pointer on the stack

to get the data follow the pointer

now heres a thing pushing to stack is fast as allocator never has to search for a place to store new data as its always at the top of stack

but allocator must first find a big enough space to build the datat then perform the backkeeping to prepare for the next allocation

accesing the stored or allocated data in heap its slow you have to follow pointer

proccessor can do its job better if works on close to data or other data stack rather than far away which is heap

functions when u call a function the values are passed into the funcn including pointers to data on hheap and thee funcns local variables get pushed on to stack when func is executed values get popped out off stack simple

ownerships purpose - to manage heap data 

keeping track of what code is using what data on the heap minimalzisng the amount of duplicate data on the hheap and also cleaning up unused datat on the heap

ownership rules 
each value has an owner 
there can be only one owner at a time
when the owner goes out of scope value is dropped